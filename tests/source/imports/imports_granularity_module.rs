// rustfmt-imports_granularity: Module

use a::{b::c, d::e};
use a::{
    f,
    g::{h, i},
};
use a::{
    j::{
        self,
        k::{self, l},
        m,
    },
    n::{o::p, q},
};
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

use b::c;
use b::d;
use b::e;
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
use b::{
    f::g,
    h::{i, j}, /* After b::h group */
};
use b::{
    /* Before b::l group */ l::{self, m, n::o, p::*},
    q,
};

use c;
use d;

use {library1, library2 as lib2, library3};
