// rustfmt-version: Two

//  Original issue test case

#[cfg(windows)]
fn main() {
    use common_util::get_version_for_rc;
    use std::env;
    use winres::{VersionInfo, WindowsResource};

    println!("cargo:rerun-if-changed=favicon.ico");
    println!("cargo:rerun-if-changed=Cargo.toml"); // rerun when version changed
    /*let profile = env::var("PROFILE").unwrap();
    if profile == "release"*/
    {
        let mut res = WindowsResource::new();
        res.set_icon("favicon.ico");
        let version = get_version_for_rc!();
        res.set_version_info(VersionInfo::PRODUCTVERSION, version);
        res.set_version_info(VersionInfo::FILEVERSION, version);
        res.compile().unwrap();
    }
}

#[cfg(not(windows))]
fn main() {}

// Other test cases

fn main() {
    let a = 1; // One line comment
    /* Block comment */
    true
}

fn main() {
    let a = 1; /* Block comment */
    // One line comment
    true
}

fn main() {
    let a = 1; // One line comment
    /* Block comment */
}

fn main() {
    let a = 1; /* Block comment */
    // One line comment
}

fn main() {
    // One line comment
    /* Block comment */
    true
}

fn main() {
    /* Block comment */
    // One line comment
    true
}

fn main() {
    // One line comment
    /* Block comment */
}

fn main() {
    /* Block comment */
    // One line comment
}
