// A comment inside the parens of a restricted visibility must survive
// formatting. The path alone cannot round-trip it.

pub(crate /* important comment */) fn a() {}

pub(crate // important comment
) fn a_line() {
}

pub(in /* which */ foo::bar) fn b() {}

pub(in // which
    foo::bar) fn b_line() {
}

pub(super /* up */) fn c() {}

pub(super // up
) fn c_line() {
}

pub(self /* here */) fn d() {}

pub(self // here
) fn d_line() {
}

pub(/* leading */ crate) fn e() {}

pub(// leading
    crate) fn e_line() {
}

pub(crate) fn no_comment() {}

pub(crate /* mod */) mod m {}

pub(crate // mod
) mod m_line {}

pub(crate /* use */) use std::io;

pub(crate // use
) use std::fmt;

struct S {
    pub(crate /* field */) field: u32,

    pub(crate // field
    ) field_line: u32,
}

// Comments that span more than one line are kept as-is too.
pub(crate /* multi
   line */) fn multi_line_block() {
}

pub(crate // multi
    // line
) fn multi_line_single() {
}

pub(in /* one */ foo::bar /* two */) fn two_comments() {}

pub(in foo::bar // one
    // two
) fn two_comments_line() {
}
