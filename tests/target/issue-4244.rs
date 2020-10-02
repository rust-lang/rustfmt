pub struct SS {}

pub type A /* A Comment */ = SS;

pub type B // Comment
    // B
    = SS;

pub type C
    /* Comment C */
    = SS;

pub trait D<T> {
    type E /* Comment E */ = SS;
}
