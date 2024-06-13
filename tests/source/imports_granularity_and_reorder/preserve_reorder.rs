// rustfmt-imports_granularity: Preserve
// rustfmt-reorder_imports: true

use a::b::c::d::e;

use a::b::{
d, c,
};

use a::b::{
w::{d, c},
};

use a::b::{
z, x, y,
u::{b, a},
w::{d, c},
};

use z123::foo;
use z123::baz;

use z123::baz;
use z123::foo;
