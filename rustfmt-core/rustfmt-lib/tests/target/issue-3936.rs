// rustfmt-hard_tabs: true

macro_rules! m {
	($a:expr) => {
		if $a {
			return;
		}
	};
}
