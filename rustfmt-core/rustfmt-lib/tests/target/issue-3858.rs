// rustfmt-hard_tabs: true

macro_rules! member_mut {
	($self:expr, $member:expr) => {{
		use self::Member::*;
		let r = &mut *$self;
		match $member {
			A => &mut r.a,
			B => &mut r.b,
		}
	}};
}
