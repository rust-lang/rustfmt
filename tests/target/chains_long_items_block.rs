// rustfmt-indent_style: Block

// https://github.com/rust-lang/rustfmt/issues/3863
fn issue_3863() {
    foo("This text is under the max_width limit, and shouldn't cause any problems on its own.")
        .long("But this line is extra long, and doesn't fit within 100 max_width. 1234567890123456789 aBcDeFgHiJ")
        .baz()
        .collect()
        .unwrap();
}

// https://github.com/rust-lang/rustup.rs/pull/2097
fn deeply_nested() {
    let mut app = App::new("rustup")
        .version(common::version())
        .about("The Rust toolchain installer")
        .after_help(RUSTUP_HELP)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a command with an environment configured for a given toolchain")
                .after_help(RUN_HELP)
                .setting(AppSettings::TrailingVarArg)
                .arg(
                    Arg::with_name("install")
                        .help("Install the requested toolchain if needed")
                        .long("install"),
                )
                .arg(
                    Arg::with_name("toolchain")
                        .help(TOOLCHAIN_ARG_HELP)
                        .required(true),
                )
                .arg(
                    Arg::with_name("command")
                        .required(true)
                        .multiple(true)
                        .use_delimiter(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("which")
                .about("Display which binary will be run for a given command")
                .arg(Arg::with_name("command").required(true))
                .arg(
                    Arg::with_name("toolchain")
                        .help(TOOLCHAIN_ARG_HELP)
                        .long("toolchain")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("doc")
                .alias("docs")
                .about("Open the documentation for the current toolchain")
                .after_help(DOC_HELP)
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .help("Only print the path to the documentation"),
                )
                .args(
                    &DOCS_DATA
                        .iter()
                        .map(|(name, help_msg, _)| Arg::with_name(name).long(name).help(help_msg))
                        .collect::<Vec<_>>(),
                )
                .arg(
                    Arg::with_name("toolchain")
                        .help(TOOLCHAIN_ARG_HELP)
                        .long("toolchain")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("page").args(
                        &DOCS_DATA
                            .iter()
                            .map(|(name, _, _)| *name)
                            .collect::<Vec<_>>(),
                    ),
                )
                .arg(
                    Arg::with_name("topic")
                        .help("Topic such as 'core', 'fn', 'usize', 'eprintln!', 'core::arch', 'alloc::format!', 'std::fs', 'std::fs::read_dir', 'std::io::Bytes', 'std::iter::Sum', 'std::io::error::Result' etc..."),
                ),
        );
}

fn long_parent() {
    // Args that do not fit
    let bar = baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    // Long element no args
    let foo = looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg()
        .foo()
        .bar()
        .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
        .foo()
        .bar()
        .baz();

    // Long element with args that fit
    let bar = looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg("ffffffffffffffffffffffffffffffffffff")
        .foo()
        .bar()
        .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("ffffffffffffffffffffffffffffffffffff")
        .foo()
        .bar()
        .baz();
}

fn long_inner() {
    // Args that do not fit
    let bar = bar()
        .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    qux()
        .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    // Long element with args that fit
    let bar = bar()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
            "ffffffffffffffffffffffffffffffffffff",
        )
        .foo()
        .bar()
        .baz();

    qux()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
            "ffffffffffffffffffffffffffffffffffff",
        )
        .foo()
        .bar()
        .baz();

    // Long element no args
    let foo = bar()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
        .foo()
        .bar()
        .baz();

    qux()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
        .foo()
        .bar()
        .baz();
}

fn long_tail() {
    // Args that do not fit
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

    // Long element with args that fit
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
            "abc123def456",
        );

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
            "abc123def456",
        );

    // Long element no args
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff();

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff();
}

fn raw_str_lit() {
    fpo.foo()
        .bar
        .baz
        .quz
        .adfasdfasdfasdfasdfasdfasdfasffffffffffffffffffffffffffffffffffdfasf(
            r#"
    if foo {
        a();
    }
    else {
        b();
    }
    "#
            .trim(),
        );

    fpo.foo()
        .bar
        .baz
        .quz
        .a999999999999999999999999999999999999999999999999999999999999999(
            r#"
    if foo {
        a();
    }
    else {
        b();
    }
    "#
            .trim()
            .foo()
            .bar
            .baz
            .qux()
            .unwrap(),
        );
}

fn comments() {
    foo // foo
        // comment after parent
        .x
        .y
        // comment 1
        .bar("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk") // comment after bar()
        // comment 2
        .foobar
        // comment after
        // comment 3
        .baz(x, y, z);
}
