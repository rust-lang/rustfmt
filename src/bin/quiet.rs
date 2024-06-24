use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use rustfmt_nightly::{load_config, CliOptions, Config, Input, Session};

fn main() {
    let mut args = std::env::args();
    let Some(_arg0) = args.next() else {
        std::process::exit(1);
    };
    let Some(filename) = args.next() else {
        std::process::exit(1);
    };
    let filename: PathBuf = filename.into();
    let opt_config = args.next().map(PathBuf::from);

    let config = if let Some(ref config_file_path) = opt_config {
        load_config(Some(config_file_path), None::<NullOptions>)
            .expect("`rustfmt.toml` not found")
            .0
    } else {
        read_config(&filename)
    };

    let input = Input::File(filename);
    let mut session = Session::<Blackhole>::new(config, None);
    let _ = session.format(input).unwrap();
}

struct Blackhole;
impl Write for Blackhole {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct NullOptions;

impl CliOptions for NullOptions {
    fn apply_to(self, _: &mut Config) {
        unreachable!();
    }
    fn config_path(&self) -> Option<&Path> {
        unreachable!();
    }
}

fn read_config(filename: &Path) -> Config {
    let sig_comments = read_significant_comments(filename);
    // Look for a config file. If there is a 'config' property in the significant comments, use
    // that. Otherwise, if there are no significant comments at all, look for a config file with
    // the same name as the test file.
    let mut config = if !sig_comments.is_empty() {
        load_config(
            sig_comments.get("config").map(Path::new),
            None::<NullOptions>,
        )
        .map(|(config, _)| config)
        .unwrap_or_default()
    } else {
        load_config(
            filename.with_extension("toml").file_name().map(Path::new),
            None::<NullOptions>,
        )
        .map(|(config, _)| config)
        .unwrap_or_default()
    };

    for (key, val) in &sig_comments {
        if key != "target" && key != "config" && key != "unstable" {
            config.override_value(key, val);
        }
    }

    config
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
