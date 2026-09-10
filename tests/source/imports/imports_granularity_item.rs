// rustfmt-imports_granularity: Item

use a::{b, c, d};
use a::{f::g, h::{i, j}};
use a::{l::{self, m, n::o, p::*}};
use a::q::{self};

use b::{f::g, h::{i, j} /* After b::h group */};
use b::e;
use b::{/* Before b::l group */ l::{self, m, n::o, p::*}, q};
use b::d;
use b::r; // After b::r
use b::q::{self /* After b::q::self */};
use b::u::{
    a,
    b,
};
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
use b::c;

use c::{a::b, c::d, c::e};
#[cfg(unix)]
use c::{a::b, c::d, c::e};
#[cfg(windows)]
use c::{a::b, c::d, c::e};
// my comment
use c::a::b;
