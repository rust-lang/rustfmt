// Case 1: exact issue repro (comment inside bounds + before-where clause).
pub trait Trait {
    type I: Iterator<
        // This is an item
        Item = Self,
    >
    where
        Self: Copy;
}
