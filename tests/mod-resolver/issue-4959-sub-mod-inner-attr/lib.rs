mod b;
mod a;
#[rustfmt::skip]
mod c;
mod d;

#[cfg(linux)]
#[path ="e1.rs"]
mod e;
#[cfg(not(linux))]
#[path ="e2.rs"]
mod e;
