// rustfmt-normalize_comments: true

use path::{C,/*A*/ A, B /* B */, self /* self */};

use {ab, ac, aa, Z, b};

// The sort order shall follow versionsort
use {u8, u128, u64, u16, u32};
use {v1, v0200, v0030, v0002, v02000, v02001};
// Order by alias should use versionsort too
use {crate as crate10, crate as crate2, crate as crate1};
