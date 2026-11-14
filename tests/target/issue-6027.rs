// rustfmt-imports_granularity: One

use {
    bar::{
        a,
        b::{self as B, f},
    },
    baz::{c as x, c as y},
    qux::{h, i, i as j},
};
