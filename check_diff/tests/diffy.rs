use diffy::{self, create_patch};

#[test]
fn diffy_test_diff() {
    let original = "The quick brown fox jumps over the lazy dog";
    let modified = "The quick brown fox jumps over the LAZY dog";

    let patch = create_patch(original, modified);
    // diffy uses hunks which indicates the lines that are different
    assert_eq!(patch.hunks().is_empty(), false);
    // hence regardless, patch.to_string() will never be empty
    assert_eq!(patch.to_string().is_empty(), false);
}

#[test]
fn diffy_test_no_diff() {
    let original = "The quick brown fox jumps over the lazy dog";

    let patch = create_patch(original, original);
    assert_eq!(patch.hunks().is_empty(), true);
    assert_eq!(patch.to_string().is_empty(), false);
}
