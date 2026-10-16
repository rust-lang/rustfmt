// rustfmt-imports_granularity: Module
// rustfmt-group_imports: One
// rustfmt-reorder_imports: false

use a::b::c;
use a::d::e;
use a::f;
use a::g::{h, i};
use a::j::{self, m};
use a::j::k::{self, l};
use a::n::o::p;
use a::n::q;
pub use a::r::s;
pub use a::t;
use b::{self, c, d, e};
use b::c::d;
#[cfg(test)]
use foo::{a::b, c::d};
use foo::e;
use bar::{
    // comment
    a::b,
    // more comment
    c::d,
    e::f,
};
use b::{
    f::g,
    h::{i, j}, /* After b::h group */
};
use b::{
    /* Before b::l group */ l::{self, m, n::o, p::*},
    q,
};
use b::r; // After b::r
use b::q::{self /* After b::q::self */};
use b::u::{a, b};
use b::t::{
    // Before b::t::a
    a,
    b,
};
use b::s::{
    a,
    b, // After b::s::b
};
use b::v::{
    // Before b::v::a
    a,
    // Before b::v::b
    b,
};
use b::t::{/* Before b::t::self */ self};
