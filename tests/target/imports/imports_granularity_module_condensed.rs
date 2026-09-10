// rustfmt-imports_granularity: ModuleCondensed

use a::g::{h, i};
use a::j::k::{self, l};
use a::j::{self, m};
use a::n::{o::p, q};
use a::{b::c, d::e, f};
pub use a::{r::s, t};
use b::{self, c::d};

use foo::e;
#[cfg(test)]
use foo::{a::b, c::d};

use bar::{
    // comment
    a::b,
    // more comment
    c::d,
    e::f,
};

use b::q::{self /* After b::q::self */};
use b::r; // After b::r
use b::s::{
    a,
    b, // After b::s::b
};
use b::t::{/* Before b::t::self */ self};
use b::t::{
    // Before b::t::a
    a,
    b,
};
use b::u::{a, b};
use b::v::{
    // Before b::v::a
    a,
    // Before b::v::b
    b,
};
use b::{c, d, e};
use b::{
    f::g,
    h::{i, j}, /* After b::h group */
};
use b::{
    /* Before b::l group */ l::{self, m, n::o, p::*},
    q,
};
