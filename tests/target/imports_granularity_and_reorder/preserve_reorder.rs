// rustfmt-imports_granularity: Preserve
// rustfmt-reorder_imports: true

use a::b::c::d::e;

use a::b::{c, d};

use a::b::w::{c, d};

use a::b::{
    u::{a, b},
    w::{c, d},
    x, y, z,
};

use z123::baz;
use z123::foo;

use z123::baz;
use z123::foo;
