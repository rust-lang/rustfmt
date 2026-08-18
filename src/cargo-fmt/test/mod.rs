use super::*;

mod message_format;
mod targets;

#[test]
fn default_options() {
    let empty: Vec<String> = vec![];
    let o = Opts::parse_from(&empty);
    assert_eq!(false, o.quiet);
    assert_eq!(false, o.verbose);
    assert_eq!(false, o.version);
    assert_eq!(false, o.check);
    assert_eq!(empty, o.packages);
    assert_eq!(empty, o.rustfmt_options);
    assert_eq!(false, o.format_all);
    assert_eq!(None, o.manifest_path);
    assert_eq!(None, o.message_format);
}

#[test]
fn good_options() {
    let o = Opts::parse_from([
        "test",
        "-q",
        "-p",
        "p1",
        "-p",
        "p2",
        "--message-format",
        "short",
        "--check",
        "--",
        "--edition",
        "2018",
    ]);
    assert_eq!(true, o.quiet);
    assert_eq!(false, o.verbose);
    assert_eq!(false, o.version);
    assert_eq!(true, o.check);
    assert_eq!(vec!["p1", "p2"], o.packages);
    assert_eq!(vec!["--edition", "2018"], o.rustfmt_options);
    assert_eq!(false, o.format_all);
    assert_eq!(Some(String::from("short")), o.message_format);
}

#[test]
fn unexpected_option() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "unexpected"])
            .is_err()
    );
}

#[test]
fn unexpected_flag() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "--flag"])
            .is_err()
    );
}

#[test]
fn mandatory_separator() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "--emit"])
            .is_err()
    );
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "--", "--emit"])
            .is_ok()
    );
}

#[test]
fn multiple_packages_one_by_one() {
    let o = Opts::parse_from([
        "test",
        "-p",
        "package1",
        "--package",
        "package2",
        "-p",
        "package3",
    ]);
    assert_eq!(3, o.packages.len());
}

#[test]
fn multiple_packages_grouped() {
    let o = Opts::parse_from([
        "test",
        "--package",
        "package1",
        "package2",
        "-p",
        "package3",
        "package4",
    ]);
    assert_eq!(4, o.packages.len());
}

#[test]
fn empty_packages_1() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "-p"])
            .is_err()
    );
}

#[test]
fn empty_packages_2() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "-p", "--", "--check"])
            .is_err()
    );
}

#[test]
fn empty_packages_3() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "-p", "--verbose"])
            .is_err()
    );
}

#[test]
fn empty_packages_4() {
    assert!(
        Opts::command()
            .try_get_matches_from(["test", "-p", "--check"])
            .is_err()
    );
}

#[test]
fn rustfmt_files_are_split_at_the_command_line_limit() {
    let rustfmt = Path::new("rustfmt");
    let fixed_args = [OsString::from("--edition"), OsString::from("2021")];
    let files = [
        Path::new("first.rs"),
        Path::new("second.rs"),
        Path::new("third.rs"),
    ];
    let two_file_limit = command_line_program_len(rustfmt)
        + fixed_args
            .iter()
            .map(|arg| command_line_arg_len(arg))
            .sum::<usize>()
        + files[..2]
            .iter()
            .map(|file| command_line_arg_len(file.as_os_str()))
            .sum::<usize>();

    let batches = rustfmt_file_batches(rustfmt, &files, &fixed_args, two_file_limit);

    assert_eq!(batches, vec![vec![files[0], files[1]], vec![files[2]]]);
}

#[test]
fn rustfmt_file_batching_keeps_an_oversized_file() {
    let rustfmt = Path::new("rustfmt");
    let files = [Path::new("a-file-that-is-longer-than-the-limit.rs")];

    let batches = rustfmt_file_batches(rustfmt, &files, &[], 1);

    assert_eq!(batches, vec![vec![files[0]]]);
}

#[cfg(windows)]
#[test]
fn windows_command_line_length_matches_rust_quoting() {
    assert_eq!(command_line_arg_len(OsStr::new("plain")), 6);
    assert_eq!(command_line_arg_len(OsStr::new("")), 3);
    assert_eq!(command_line_arg_len(OsStr::new("has space")), 12);
    assert_eq!(command_line_arg_len(OsStr::new(r#"a\"b"#)), 7);
    assert_eq!(command_line_arg_len(OsStr::new(r#"a \"b"#)), 10);
    assert_eq!(command_line_arg_len(OsStr::new(r#"a \"#)), 7);
    assert_eq!(command_line_program_len(Path::new("rustfmt")), 9);
}
