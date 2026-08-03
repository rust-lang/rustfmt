// rustfmt-imports_granularity: ModuleCondensed
// rustfmt-group_imports: One
// rustfmt-reorder_imports: false

use a::{b::c, d::e, f};
use a::g::{h, i};
use a::j::{self, m};
use a::j::k::{self, l};
use a::n::{o::p, q};
pub use a::{r::s, t};
use b::{self, c, c::d, d, e};
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
