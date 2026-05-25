// rustfmt-imports_granularity: Item

/// rustdoc comment
use a::{a, b, c};

// standard comment
use b::{a, b, c};

#[doc = "also rustdoc comment"]
use c::{a, b, c};
