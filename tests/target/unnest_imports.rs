// rustfmt-unnest_imports: true

use a::b::c;
use a::d::e;
use a::f;
use a::g::{h, i};
use a::j::k::{self, l};
use a::j::{self, m};
use a::n::o::p;
use a::n::q;
pub use a::r::s;
pub use a::t;

#[cfg(test)]
use foo::{a::b, c::d};

use bar::e::f;
use bar::{
    // comment
    a::b,
    // more comment
    c::d,
};
