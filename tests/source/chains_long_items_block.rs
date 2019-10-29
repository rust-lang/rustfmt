// https://github.com/rust-lang/rustfmt/issues/3863
fn f() {
    foo("This text is under the max_width limit, and shouldn't cause any problems on its own.").long("But this line is extra long, and doesn't fit within 100 max_width. 1234567890123456789 aBcDeFgHiJ").baz().collect().unwrap();
}
