fn main() {
    generate_tests_markdown_tests().unwrap()
}

#[cfg(not(feature = "gen-tests"))]
fn generate_tests_markdown_tests() -> std::io::Result<()> {
    Ok(())
}

#[cfg(feature = "gen-tests")]
fn generate_tests_markdown_tests() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::PathBuf;

    let spec_folder = "./tests/spec/";
    let test_folder = "./tests/";

    let spec_files = [
        (
            "",
            "commonmark_v0_30_spec.json",
            "https://spec.commonmark.org/0.30/",
        ),
        ("gfm_", "gfm_spec.json", "https://github.github.com/gfm/"),
    ];

    for (prefix, spec, url) in spec_files {
        let input_file = format!("{spec_folder}{spec}");
        let mut output_file = PathBuf::from(format!("{test_folder}{spec}"));
        output_file.set_extension("rs");

        let test_cases: Vec<TestCase<'_>> = serde_json::from_reader(File::open(&input_file)?)?;
        let mut output = BufWriter::new(File::create(&output_file)?);

        write_test_cases(&mut output, prefix, test_cases, url)
            .expect("generated test case successfully");
    }

    Ok(())
}

#[cfg(feature = "gen-tests")]
#[derive(Debug, serde::Deserialize)]
struct TestCase<'a> {
    #[serde(rename(deserialize = "markdown"))]
    input: std::borrow::Cow<'a, str>,
    #[serde(rename(deserialize = "formattedMarkdown"))]
    output: Option<std::borrow::Cow<'a, str>>,
    #[serde(rename(deserialize = "example"))]
    id: usize,
    section: std::borrow::Cow<'a, str>,
    #[serde(default)]
    skip: bool,
    #[serde(default = "default_test", rename(deserialize = "testMacro"))]
    test_macro: std::borrow::Cow<'a, str>,
    comment: Option<std::borrow::Cow<'a, str>>,
}

#[cfg(feature = "gen-tests")]
fn default_test() -> std::borrow::Cow<'static, str> {
    // Name of the test macro to use
    "test_identical_markdown_events".into()
}

#[cfg(feature = "gen-tests")]
fn write_test_cases<W>(
    writer: &mut W,
    prefix: &str,
    test_cases: Vec<TestCase<'_>>,
    url: &str,
) -> std::io::Result<()>
where
    W: std::io::Write,
{
    write!(writer, "// @generated\n")?;
    write!(writer, "// generated running `cargo build -F gen-tests`\n")?;
    write!(
        writer,
        "// test macros are defined in tests/common/mod.rs\n"
    )?;
    write!(writer, "mod common;\n")?;

    for test_case in test_cases.into_iter() {
        write_test_case(writer, prefix, test_case, url)?;
    }
    Ok(())
}

#[cfg(feature = "gen-tests")]
fn write_test_case<W: std::io::Write>(
    writer: &mut W,
    prefix: &str,
    test_case: TestCase<'_>,
    url: &str,
) -> std::io::Result<()> {
    let url = if url.ends_with("/") {
        format!("{}#example-{}", url, test_case.id)
    } else {
        format!("{}/#example-1{}", url, test_case.id)
    };

    let replace_tab_chars = test_case.input.replace('→', "\t");
    let input = replace_tab_chars.trim_end_matches('\n');

    if let Some(comment) = test_case.comment {
        write!(writer, "\n// {comment}")?;
    }

    if test_case.skip {
        write!(writer, "\n#[ignore]")?;
    }

    write!(
        writer,
        r##"
#[test]
fn {}markdown_{}_{}() {{
    // {}
    {}!("##,
        prefix,
        test_case
            .section
            .to_lowercase()
            .replace(char::is_whitespace, "_")
            .replace("(", "")
            .replace(")", ""),
        test_case.id,
        url,
        test_case.test_macro,
    )?;

    let has_trailing_whitespace = input.lines().any(|l| l.ends_with(char::is_whitespace));
    if has_trailing_whitespace {
        write!(writer, "{:?}", input)?;
    } else {
        write!(writer, "r##\"{}\"##", input)?;
    }
    if let Some(expected_output) = test_case.output {
        let has_trailing_whitespace = expected_output
            .lines()
            .any(|l| l.ends_with(char::is_whitespace));
        if has_trailing_whitespace {
            write!(writer, ",{:?}", expected_output)?;
        } else {
            write!(writer, ",r##\"{}\"##", expected_output)?;
        }
    }
    write!(writer, ");")?;
    write!(writer, "\n}}\n")?;
    Ok(())
}
