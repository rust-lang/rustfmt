// rustfmt-hard_tabs: true

macro_rules! foo {
	($bar: expr, $t: ty) => {
		$bar(|x| {
			if x {
				None;
			} else {
				None;
			}
		})
	};
}
