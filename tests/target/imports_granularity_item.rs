// rustfmt-imports_granularity: Item

use a::b::c;
use a::d::e;
use a::f;
use a::g::h;
use a::g::i;
use a::j;
use a::j::k;
use a::j::k::l;
use a::j::m;
use a::n::o::p;
use a::n::q;
pub use a::r::s;
pub use a::t;

use foo::e;
#[cfg(test)]
use foo::{a::b, c::d};

use bar::a::b;
use bar::c::d;
use bar::e::f;
