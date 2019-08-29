// rustfmt-normalize_comments: true

use path::{self /* self */, /* A */ A, B /* B */, C};

use {aa, ab, ac, b, Z};

// The sort order shall follow versionsort
use {u8, u16, u32, u64, u128};
use {v0002, v0030, v0200, v02000, v02001, v1};
