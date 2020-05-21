// rustfmt-hard_tabs: true
// rustfmt-single_line_if_else_max_width: 10

macro_rules! foo {
	($bar: expr, $t: ty) => {
		$bar(|x| {
			if x {
				None
			} else {
				None
			}
		})
	};
}
