// A comment inside the parens of a restricted visibility must survive
// formatting. The path alone cannot round-trip it.

pub(crate /* important comment */) fn a() {}

pub(in /* which */ foo::bar) fn b() {}

pub(super /* up */) fn c() {}

pub(self /* here */) fn d() {}

pub(/* leading */ crate) fn e() {}

pub(crate) fn no_comment() {}

pub(crate /* mod */) mod m {}

pub(crate /* use */) use std::io;

struct S {
    pub(crate /* field */) field: u32,
}

// Multi-line comments are kept as-is too.
pub(crate /* multi
   line */) fn multi_line_block() {
}

pub(crate //  line
) fn line_comment() {
}

pub(in /* one */ foo::bar /* two */) fn two_comments() {}
