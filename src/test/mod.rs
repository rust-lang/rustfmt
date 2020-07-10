use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::str::Chars;
use std::thread;

use crate::emitter::rustfmt_diff::{make_diff, print_diff, Mismatch, ModifiedChunk, OutputWriter};

use crate::config::{Config, FileName, NewlineStyle};
use crate::{
    emitter::{emit_format_report, Color, EmitMode, EmitterConfig},
    format,
    formatting::modules::{ModuleResolutionError, ModuleResolutionErrorKind},
    is_nightly_channel, FormatReport, FormatReportFormatterBuilder, Input, OperationError,
    OperationSetting,
};

mod configuration_snippet;

const DIFF_CONTEXT_SIZE: usize = 3;

// A list of files on which we want to skip testing.
const SKIP_FILE_WHITE_LIST: &[&str] = &[
    "issue-3434/no_entry.rs",
    "issue-3665/sub_mod.rs",
    // Testing for issue-3779
    "issue-3779/ice.rs",
    // These files and directory are a part of modules defined inside `cfg_if!`.
    "cfg_if/mod.rs",
    "cfg_if/detect",
    "issue-3253/foo.rs",
    "issue-3253/bar.rs",
    "issue-3253/paths",
    // These files and directory are a part of modules defined inside `cfg_attr(..)`.
    "cfg_mod/dir",
    "cfg_mod/bar.rs",
    "cfg_mod/foo.rs",
    "cfg_mod/wasm32.rs",
    // We want to ensure `recursive` is working correctly, so do not test
    // these files directly
    "configs/recursive/disabled/foo.rs",
    "configs/recursive/enabled/foo.rs",
    "mods-relative-path/mod_b.rs",
];

fn init_log() {
    let _ = env_logger::builder().is_test(true).try_init();
}

struct TestSetting {
    /// The size of the stack of the thread that run tests.
    stack_size: usize,
}

impl Default for TestSetting {
    fn default() -> Self {
        TestSetting {
            stack_size: 8_388_608, // 8MB
        }
    }
}

fn run_test_with<F>(test_setting: &TestSetting, f: F)
where
    F: FnOnce(),
    F: Send + 'static,
{
    thread::Builder::new()
        .stack_size(test_setting.stack_size)
        .spawn(f)
        .expect("Failed to create a test thread")
        .join()
        .expect("Failed to join a test thread")
}

fn is_subpath<P>(path: &Path, subpath: &P) -> bool
where
    P: AsRef<Path>,
{
    (0..path.components().count())
        .map(|i| {
            path.components()
                .skip(i)
                .take(subpath.as_ref().components().count())
        })
        .any(|c| c.zip(subpath.as_ref().components()).all(|(a, b)| a == b))
}

fn is_file_skip(skip_file_white_list: &[&str], path: &Path) -> bool {
    skip_file_white_list
        .iter()
        .any(|file_path| is_subpath(path, file_path))
}

// Returns a `Vec` containing `PathBuf`s of files with an  `rs` extension in the
// given path. The `recursive` argument controls if files from subdirectories
// are also returned.
fn get_test_files(path: &Path, recursive: bool, skip_file_white_list: &[&str]) -> Vec<PathBuf> {
    assert!(path.exists(), "{} does not exist", path.display());

    let mut files = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)
            .unwrap_or_else(|_| panic!("couldn't read directory {}", path.display()))
        {
            let entry = entry.expect("couldn't get `DirEntry`");
            let path = entry.path();
            if path.is_dir() && recursive {
                files.append(&mut get_test_files(&path, recursive, skip_file_white_list));
            } else if path.extension().map_or(false, |f| f == "rs")
                && !is_file_skip(skip_file_white_list, &path)
            {
                files.push(path);
            }
        }
    }
    files
}

fn verify_config_used(path: &Path, config_name: &str) {
    for entry in
        fs::read_dir(path).unwrap_or_else(|_| panic!("couldn't read {} directory", path.display()))
    {
        let entry = entry.expect("couldn't get directory entry");
        let path = entry.path();
        if path.extension().map_or(false, |f| f == "rs") {
            // check if "// rustfmt-<config_name>:" appears in the file.
            let filebuf = BufReader::new(
                fs::File::open(&path)
                    .unwrap_or_else(|_| panic!("couldn't read file {}", path.display())),
            );
            assert!(
                filebuf
                    .lines()
                    .map(Result::unwrap)
                    .take_while(|l| l.starts_with("//"))
                    .any(|l| l.starts_with(&format!("// rustfmt-{}", config_name))),
                format!(
                    "config option file {} does not contain expected config name",
                    path.display()
                )
            );
        }
    }
}

#[test]
fn verify_config_test_names() {
    init_log();
    for path in &[
        Path::new("tests/source/configs"),
        Path::new("tests/target/configs"),
    ] {
        for entry in fs::read_dir(path).expect("couldn't read configs directory") {
            let entry = entry.expect("couldn't get directory entry");
            let path = entry.path();
            if path.is_dir() {
                let config_name = path.file_name().unwrap().to_str().unwrap();

                // Make sure that config name is used in the files in the directory.
                verify_config_used(&path, config_name);
            }
        }
    }
}

// This writes to the terminal using the same approach (via `term::stdout` or
// `println!`) that is used by `rustfmt::rustfmt_diff::print_diff`. Writing
// using only one or the other will cause the output order to differ when
// `print_diff` selects the approach not used.
fn write_message(msg: &str) {
    let mut writer = OutputWriter::new(Color::Auto);
    writer.writeln(msg, None);
}

// Integration tests. The files in `tests/source` are formatted and compared
// to their equivalent in `tests/target`. The target file and config can be
// overridden by annotations in the source file. The input and output must match
// exactly.
#[test]
fn system_tests() {
    init_log();
    run_test_with(&TestSetting::default(), || {
        // Get all files in the tests/source directory.
        let files = get_test_files(Path::new("tests/source"), true, SKIP_FILE_WHITE_LIST);
        let (_reports, count, fails) = check_files(files, &None);

        // Display results.
        println!("Ran {} system tests.", count);
        assert_eq!(fails, 0, "{} system tests failed", fails);
        assert!(
            count >= 300,
            "Expected a minimum of {} system tests to be executed",
            300
        )
    });
}

#[test]
fn checkstyle_test() {
    init_log();
    let filename = "tests/writemode/source/fn-single-line.rs";
    let expected_filename = "tests/writemode/target/checkstyle.xml";
    assert_output(Path::new(filename), Path::new(expected_filename));
}

#[test]
fn json_test() {
    init_log();
    let filename = "tests/writemode/source/json.rs";
    let expected_filename = "tests/writemode/target/output.json";
    assert_output(Path::new(filename), Path::new(expected_filename));
}

#[test]
fn modified_test() {
    init_log();
    use std::io::BufRead;

    // Test "modified" output
    let filename = "tests/writemode/source/modified.rs";
    let config = Config::default();
    let setting = OperationSetting::default();

    let report = format(Input::File(filename.into()), &config, setting).unwrap();
    let mut data = vec![];
    emit_format_report(
        report,
        &mut data,
        EmitterConfig {
            emit_mode: EmitMode::ModifiedLines,
            ..EmitterConfig::default()
        },
    )
    .unwrap();

    let mut lines = data.lines();
    let mut chunks = Vec::new();
    while let Some(Ok(header)) = lines.next() {
        // Parse the header line
        let values: Vec<_> = header
            .split(' ')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        assert_eq!(values.len(), 3);
        let line_number_orig = values[0];
        let lines_removed = values[1];
        let num_added = values[2];
        let mut added_lines = Vec::new();
        for _ in 0..num_added {
            added_lines.push(lines.next().unwrap().unwrap());
        }
        chunks.push(ModifiedChunk {
            line_number_orig,
            lines_removed,
            lines: added_lines,
        });
    }

    assert_eq!(
        chunks,
        vec![
            ModifiedChunk {
                line_number_orig: 4,
                lines_removed: 4,
                lines: vec!["fn blah() {}".into()],
            },
            ModifiedChunk {
                line_number_orig: 9,
                lines_removed: 6,
                lines: vec!["#[cfg(a, b)]".into(), "fn main() {}".into()],
            },
        ],
    );
}

// Helper function for comparing the results of rustfmt
// to a known output file generated by one of the write modes.
fn assert_output(source: &Path, expected_filename: &Path) {
    let (config, builder, emitter_config) = read_config(source);
    let report = format_file(source, builder, config).unwrap();

    // Populate output by writing to a vec.
    let mut out = vec![];
    let _ = emit_format_report(report, &mut out, emitter_config);
    let output = String::from_utf8(out).unwrap();

    let mut expected_file = fs::File::open(&expected_filename).expect("couldn't open target");
    let mut expected_text = String::new();
    expected_file
        .read_to_string(&mut expected_text)
        .expect("Failed reading target");

    let compare = make_diff(&expected_text, &output, DIFF_CONTEXT_SIZE);
    if !compare.is_empty() {
        let mut failures = HashMap::new();
        failures.insert(source.to_owned(), compare);
        print_mismatches_default_message(failures);
        panic!("Text does not match expected output");
    }
}

// Helper function for comparing the results of rustfmt
// to a known output generated by one of the write modes.
fn assert_stdin_output(
    source: &Path,
    expected_filename: &Path,
    emit_mode: EmitMode,
    has_diff: bool,
) {
    let mut config = Config::default();
    config.set().newline_style(NewlineStyle::Unix);

    let mut source_file = fs::File::open(&source).expect("couldn't open source");
    let mut source_text = String::new();
    source_file
        .read_to_string(&mut source_text)
        .expect("Failed reading target");
    let input = Input::Text(source_text);

    // Populate output by writing to a vec.
    let mut buf: Vec<u8> = vec![];
    {
        let report = format(input, &config, OperationSetting::default()).unwrap();
        let format_has_diff = emit_format_report(
            report,
            &mut buf,
            EmitterConfig {
                emit_mode,
                ..EmitterConfig::default()
            },
        )
        .unwrap();
        assert_eq!(has_diff, format_has_diff);
    }

    let mut expected_file = fs::File::open(&expected_filename).expect("couldn't open target");
    let mut expected_text = String::new();
    expected_file
        .read_to_string(&mut expected_text)
        .expect("Failed reading target");

    let output = String::from_utf8(buf).unwrap();
    let compare = make_diff(&expected_text, &output, DIFF_CONTEXT_SIZE);
    if !compare.is_empty() {
        let mut failures = HashMap::new();
        failures.insert(source.to_owned(), compare);
        print_mismatches_default_message(failures);
        panic!("Text does not match expected output");
    }
}
// Idempotence tests. Files in tests/target are checked to be unaltered by
// rustfmt.
#[test]
fn idempotence_tests() {
    init_log();
    run_test_with(&TestSetting::default(), || {
        // these tests require nightly
        if !is_nightly_channel!() {
            return;
        }
        // Get all files in the tests/target directory.
        let files = get_test_files(Path::new("tests/target"), true, SKIP_FILE_WHITE_LIST);
        let (_reports, count, fails) = check_files(files, &None);

        // Display results.
        println!("Ran {} idempotent tests.", count);
        assert_eq!(fails, 0, "{} idempotent tests failed", fails);
        assert!(
            count >= 400,
            "Expected a minimum of {} idempotent tests to be executed",
            400
        )
    });
}

// Run rustfmt on itself. This operation must be idempotent. We also check that
// no warnings are emitted.
#[test]
fn self_tests() {
    init_log();
    // Issue-3443: these tests require nightly
    if !is_nightly_channel!() {
        return;
    }
    let skip_file_white_list = ["target", "tests"];
    let files = get_test_files(Path::new("src"), true, &skip_file_white_list);

    let (reports, count, fails) = check_files(files, &Some(PathBuf::from("rustfmt.toml")));
    let mut warnings = 0;

    // Display results.
    println!("Ran {} self tests.", count);
    assert_eq!(fails, 0, "{} self tests failed", fails);

    for format_report in reports {
        println!(
            "{}",
            FormatReportFormatterBuilder::new(&format_report).build()
        );
        warnings += format_report.warning_count();
    }

    assert_eq!(
        warnings, 0,
        "Rustfmt's code generated {} warnings",
        warnings
    );
}

#[test]
fn stdin_formatting_smoke_test() {
    init_log();
    let input = Input::Text("fn main () {}".to_owned());
    let report = format(input, &Config::default(), OperationSetting::default()).unwrap();
    assert!(!report.has_errors());
    let mut buf: Vec<u8> = vec![];
    emit_format_report(
        report,
        &mut buf,
        EmitterConfig {
            emit_mode: EmitMode::Stdout,
            ..EmitterConfig::default()
        },
    )
    .unwrap();

    #[cfg(not(windows))]
    assert_eq!(buf, b"<stdin>:\n\nfn main() {}\n");
    #[cfg(windows)]
    assert_eq!(buf, b"<stdin>:\n\nfn main() {}\r\n");
}

#[test]
fn stdin_parser_panic_caught() {
    init_log();
    // See issue #3239.
    for text in ["{", "}"].iter().cloned().map(String::from) {
        let format_result = format(
            Input::Text(text),
            &Config::default(),
            OperationSetting::default(),
        );

        assert!(format_result.err().unwrap().is_parse_error());
    }
}

/// Ensures that `EmitMode::ModifiedLines` works with input from `stdin`. Useful
/// when embedding Rustfmt (e.g. inside RLS).
#[test]
fn stdin_works_with_modified_lines() {
    init_log();
    let input = "\nfn\n some( )\n{\n}\nfn main () {}\n";
    let output = b"1 6 2\nfn some() {}\nfn main() {}\n";

    let input = Input::Text(input.to_owned());
    let mut config = Config::default();
    config.set().newline_style(NewlineStyle::Unix);
    let mut buf: Vec<u8> = vec![];
    {
        let report = format(input, &config, OperationSetting::default()).unwrap();
        let format_has_diff = emit_format_report(
            report,
            &mut buf,
            EmitterConfig {
                emit_mode: EmitMode::ModifiedLines,
                ..EmitterConfig::default()
            },
        )
        .unwrap();
        assert!(format_has_diff);
    }
    assert_eq!(buf, output);
}

/// Ensures that `EmitMode::Json` works with input from `stdin`.
#[test]
fn stdin_works_with_json() {
    init_log();
    assert_stdin_output(
        Path::new("tests/writemode/source/stdin.rs"),
        Path::new("tests/writemode/target/stdin.json"),
        EmitMode::Json,
        true,
    );
}

/// Ensures that `EmitMode::Checkstyle` works with input from `stdin`.
#[test]
fn stdin_works_with_checkstyle() {
    init_log();
    assert_stdin_output(
        Path::new("tests/writemode/source/stdin.rs"),
        Path::new("tests/writemode/target/stdin.xml"),
        EmitMode::Checkstyle,
        false,
    );
}

#[test]
fn format_lines_errors_are_reported() {
    init_log();
    let long_identifier = String::from_utf8(vec![b'a'; 239]).unwrap();
    let input = Input::Text(format!("fn {}() {{}}", long_identifier));
    let mut config = Config::default();
    config.set().error_on_line_overflow(true);
    let report = format(input, &config, OperationSetting::default()).unwrap();
    assert!(report.has_errors());
}

#[test]
fn format_lines_errors_are_reported_with_tabs() {
    init_log();
    let long_identifier = String::from_utf8(vec![b'a'; 97]).unwrap();
    let input = Input::Text(format!("fn a() {{\n\t{}\n}}", long_identifier));
    let mut config = Config::default();
    config.set().error_on_line_overflow(true);
    config.set().hard_tabs(true);
    let report = format(input, &config, OperationSetting::default()).unwrap();
    assert!(report.has_errors());
}

#[test]
fn parser_errors_in_submods_are_surfaced() {
    // See also https://github.com/rust-lang/rustfmt/issues/4126
    let filename = "tests/parser/issue-4126/lib.rs";
    let file = PathBuf::from(filename);
    let exp_mod_name = "invalid";
    let (config, operation, _) = read_config(&file);
    if let Err(OperationError::ModuleResolutionError { 0: inner }) =
        format_file(&file, operation, config)
    {
        let ModuleResolutionError { module, kind } = inner;
        assert_eq!(&module, exp_mod_name);
        if let ModuleResolutionErrorKind::ParseError { file } = kind {
            assert_eq!(file, PathBuf::from("tests/parser/issue-4126/invalid.rs"));
        } else {
            panic!("Expected parser error");
        }
    } else {
        panic!("Expected ModuleResolution operation error");
    }
}

// For each file, run rustfmt and collect the output.
// Returns the number of files checked and the number of failures.
fn check_files(files: Vec<PathBuf>, opt_config: &Option<PathBuf>) -> (Vec<FormatReport>, u32, u32) {
    let mut count = 0;
    let mut fails = 0;
    let mut reports = vec![];

    for file_name in files {
        let sig_comments = read_significant_comments(&file_name);
        if sig_comments.contains_key("unstable") && !is_nightly_channel!() {
            debug!(
                "Skipping '{}' because it requires unstable \
                 features which are only available on nightly...",
                file_name.display()
            );
            continue;
        }

        debug!("Testing '{}'...", file_name.display());

        match idempotent_check(&file_name, &opt_config) {
            Ok(ref report) if report.has_errors() => {
                print!("{}", FormatReportFormatterBuilder::new(&report).build());
                fails += 1;
            }
            Ok(report) => reports.push(report),
            Err(err) => {
                if let IdempotentCheckError::Mismatch(msg) = err {
                    print_mismatches_default_message(msg);
                }
                fails += 1;
            }
        }

        count += 1;
    }

    (reports, count, fails)
}

fn print_mismatches_default_message(result: HashMap<PathBuf, Vec<Mismatch>>) {
    for (file_name, diff) in result {
        let mismatch_msg_formatter =
            |line_num| format!("\nMismatch at {}:{}:", file_name.display(), line_num);
        print_diff(
            diff,
            &mismatch_msg_formatter,
            Color::Auto,
            Default::default(),
        );
    }

    if let Some(mut t) = term::stdout() {
        t.reset().unwrap_or(());
    }
}

fn print_mismatches<T: Fn(u32) -> String>(
    result: HashMap<PathBuf, Vec<Mismatch>>,
    mismatch_msg_formatter: T,
) {
    for (_file_name, diff) in result {
        print_diff(
            diff,
            &mismatch_msg_formatter,
            Color::Auto,
            Default::default(),
        );
    }

    if let Some(mut t) = term::stdout() {
        t.reset().unwrap_or(());
    }
}

fn read_config(filename: &Path) -> (Config, OperationSetting, EmitterConfig) {
    let sig_comments = read_significant_comments(filename);
    // Look for a config file. If there is a 'config' property in the significant comments, use
    // that. Otherwise, if there are no significant comments at all, look for a config file with
    // the same name as the test file.
    let mut config = if !sig_comments.is_empty() {
        get_config(sig_comments.get("config").map(Path::new))
    } else {
        get_config(filename.with_extension("toml").file_name().map(Path::new))
    };

    if !config.was_set().unstable_features() && !is_nightly_channel!() {
        config.override_value("unstable_features", "true");
    }

    let mut operation_setting = OperationSetting::default();
    let mut emitter_config = EmitterConfig::default();
    for (key, val) in &sig_comments {
        if key == "recursive" {
            operation_setting.recursive = val.parse::<bool>().unwrap();
        } else if key == "emit_mode" {
            emitter_config.emit_mode = val.parse::<EmitMode>().unwrap()
        } else if key != "target" && key != "config" && key != "unstable" {
            config.override_value(key, val);
            if config.is_default(key) {
                warn!("Default value {} used explicitly for {}", val, key);
            }
        }
    }

    (config, operation_setting, emitter_config)
}

fn format_file<P: Into<PathBuf>>(
    filepath: P,
    operation_setting: OperationSetting,
    config: Config,
) -> Result<FormatReport, OperationError> {
    let filepath = filepath.into();
    let input = Input::File(filepath);
    format(input, &config, operation_setting)
}

enum IdempotentCheckError {
    Mismatch(HashMap<PathBuf, Vec<Mismatch>>),
    Parse,
}

fn idempotent_check(
    filename: &PathBuf,
    opt_config: &Option<PathBuf>,
) -> Result<FormatReport, IdempotentCheckError> {
    let sig_comments = read_significant_comments(filename);
    let (config, builder, _) = if let Some(ref config_file_path) = opt_config {
        assert!(
            config_file_path.exists(),
            "{} does not exist",
            config_file_path.display()
        );

        let config = Config::from_toml_path(config_file_path).expect("`rustfmt.toml` not found");
        let builder = OperationSetting::default();
        let emitter_config = EmitterConfig::default();
        (config, builder, emitter_config)
    } else {
        read_config(filename)
    };
    let format_report = match format_file(filename, builder, config) {
        Ok(report) => report,
        Err(_) => return Err(IdempotentCheckError::Parse),
    };

    let mut write_result = HashMap::new();
    for (filename, text) in format_report.format_result() {
        if let FileName::Real(ref filename) = filename {
            write_result.insert(filename.to_owned(), text.formatted_text().to_owned());
        }
    }

    let target = sig_comments.get("target").map(|x| &(*x)[..]);

    handle_result(write_result, target).map(|_| format_report)
}

// Reads test config file using the supplied (optional) file name. If there's no file name or the
// file doesn't exist, just return the default config. Otherwise, the file must be read
// successfully.
fn get_config(config_file: Option<&Path>) -> Config {
    let config_file_name = match config_file {
        None => return Default::default(),
        Some(file_name) => {
            let mut full_path = PathBuf::from("tests/config/");
            full_path.push(file_name);
            if !full_path.exists() {
                return Default::default();
            };
            full_path
        }
    };

    let mut def_config_file = fs::File::open(config_file_name).expect("couldn't open config");
    let mut def_config = String::new();
    def_config_file
        .read_to_string(&mut def_config)
        .expect("Couldn't read config");

    Config::from_toml(&def_config, Path::new("tests/config/")).expect("invalid TOML")
}

// Reads significant comments of the form: `// rustfmt-key: value` into a hash map.
fn read_significant_comments(file_name: &Path) -> HashMap<String, String> {
    let file = fs::File::open(file_name)
        .unwrap_or_else(|_| panic!("couldn't read file {}", file_name.display()));
    let reader = BufReader::new(file);
    let pattern = r"^\s*//\s*rustfmt-([^:]+):\s*(\S+)";
    let regex = regex::Regex::new(pattern).expect("failed creating pattern 1");

    // Matches lines containing significant comments or whitespace.
    let line_regex = regex::Regex::new(r"(^\s*$)|(^\s*//\s*rustfmt-[^:]+:\s*\S+)")
        .expect("failed creating pattern 2");

    reader
        .lines()
        .map(|line| line.expect("failed getting line"))
        .filter(|line| line_regex.is_match(line))
        .filter_map(|line| {
            regex.captures_iter(&line).next().map(|capture| {
                (
                    capture
                        .get(1)
                        .expect("couldn't unwrap capture")
                        .as_str()
                        .to_owned(),
                    capture
                        .get(2)
                        .expect("couldn't unwrap capture")
                        .as_str()
                        .to_owned(),
                )
            })
        })
        .collect()
}

// Compares output to input.
// TODO: needs a better name, more explanation.
fn handle_result(
    result: HashMap<PathBuf, String>,
    target: Option<&str>,
) -> Result<(), IdempotentCheckError> {
    let mut failures = HashMap::new();

    for (file_name, fmt_text) in result {
        // If file is in tests/source, compare to file with same name in tests/target.
        let target = get_target(&file_name, target);
        let open_error = format!("couldn't open target {:?}", target);
        let mut f = fs::File::open(&target).expect(&open_error);

        let mut text = String::new();
        let read_error = format!("failed reading target {:?}", target);
        f.read_to_string(&mut text).expect(&read_error);

        // Ignore LF and CRLF difference for Windows.
        if !string_eq_ignore_newline_repr(&fmt_text, &text) {
            let diff = make_diff(&text, &fmt_text, DIFF_CONTEXT_SIZE);
            assert!(
                !diff.is_empty(),
                "Empty diff? Maybe due to a missing a newline at the end of a file?"
            );
            failures.insert(file_name, diff);
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(IdempotentCheckError::Mismatch(failures))
    }
}

// Maps source file paths to their target paths.
fn get_target(file_name: &Path, target: Option<&str>) -> PathBuf {
    if let Some(n) = file_name
        .components()
        .position(|c| c.as_os_str() == "source")
    {
        let mut target_file_name = PathBuf::new();
        for (i, c) in file_name.components().enumerate() {
            if i == n {
                target_file_name.push("target");
            } else {
                target_file_name.push(c.as_os_str());
            }
        }
        if let Some(replace_name) = target {
            target_file_name.with_file_name(replace_name)
        } else {
            target_file_name
        }
    } else {
        // This is either and idempotence check or a self check.
        file_name.to_owned()
    }
}

// Compare strings without distinguishing between CRLF and LF
fn string_eq_ignore_newline_repr(left: &str, right: &str) -> bool {
    let left = CharsIgnoreNewlineRepr(left.chars().peekable());
    let right = CharsIgnoreNewlineRepr(right.chars().peekable());
    left.eq(right)
}

struct CharsIgnoreNewlineRepr<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for CharsIgnoreNewlineRepr<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.0.next().map(|c| {
            if c == '\r' {
                if *self.0.peek().unwrap_or(&'\0') == '\n' {
                    self.0.next();
                    '\n'
                } else {
                    '\r'
                }
            } else {
                c
            }
        })
    }
}

#[test]
fn string_eq_ignore_newline_repr_test() {
    init_log();
    assert!(string_eq_ignore_newline_repr("", ""));
    assert!(!string_eq_ignore_newline_repr("", "abc"));
    assert!(!string_eq_ignore_newline_repr("abc", ""));
    assert!(string_eq_ignore_newline_repr("a\nb\nc\rd", "a\nb\r\nc\rd"));
    assert!(string_eq_ignore_newline_repr("a\r\n\r\n\r\nb", "a\n\n\nb"));
    assert!(!string_eq_ignore_newline_repr("a\r\nbcd", "a\nbcdefghijk"));
}
