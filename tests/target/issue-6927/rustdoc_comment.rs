// rustfmt-imports_granularity: Item

/// rustdoc comment
use a::a;
use a::b;
use a::c;

// standard comment
use b::a;
use b::b;
use b::c;

#[doc = "also rustdoc comment"]
use c::a;
#[doc = "also rustdoc comment"]
use c::b;
#[doc = "also rustdoc comment"]
use c::c;
