// performance tuning alternative: https://doc.rust-lang.org/alloc/raw_vec/struct.RawVec.html

/////////////////////////////////////////////////////////////////////////////////
use std::u32;

pub type TouchTy = u32;

const MAX_TOUCH_COUNT: TouchTy = u32::MAX;
