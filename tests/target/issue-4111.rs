// rustfmt-format_macro_bodies: true
// rustfmt-format_macro_matchers: true

pub macro scalar($m:ident, $t:ident) {
    pub macro $m {
        () => {
            Val::$t($t::default())
        },
        ($v: expr) => {
            Val::$t($t::new($v))
        },
    }
    pub macro a {
        () => {
            Val::$t($t::default())
        },
        ($v: expr) => {
            Val::$t($t::new($v))
        },
    }
}
