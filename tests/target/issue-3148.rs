// rustfmt-hard_tabs: true
// rustfmt-normalize_comments: true

/// ```
/// Data {
///     a: "some text data",
///     ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct Data {
	a: &str,
	b: u32,
}
