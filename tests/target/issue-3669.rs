pub trait PCG:
    self::sealed::Sealed // comment1
    + Sized                // comment2
    + Eq                   // comment3
    + Hash                 // comment4
    + Debug                // comment5
    + Clone                // comment6
    + Default              // comment7
    + Serialize            // comment8
    + for<'a> Deserialize<'a> // comment9
{
    type DoubleState: Copy                                // Note(Evrey): Because Rust is drunk. 1
        + ShrAssign<u8>                       // Note(Evrey): Because Rust is drunk. 2
        + Shl<u8, Output = Self::DoubleState> // Note(Evrey): Because Rust is drunk. 3
        + BitAnd<Output = Self::DoubleState>  // Note(Evrey): Because Rust is drunk. 4
        + BitOrAssign                         // Note(Evrey): Because Rust is drunk. 5
        + Sub<Output = Self::DoubleState>     // Note(Evrey): Because Rust is drunk. 6
        + Into<u128>                          // Note(Evrey): Because Rust is drunk. 7
        + Debug                               // Note(Evrey): Because Rust is drunk. 8
        + Eq                                  // Note(Evrey): Because Rust is drunk. 9
        + Hash                                // Note(Evrey): Because Rust is drunk. 10
        + Default                             // Note(Evrey): Because Rust is drunk. 11
        + Serialize                           // Note(Evrey): Because Rust is drunk. 12
        + for<'a> Deserialize<'a>; // Note(Evrey): Because Rust is drunk. 13
}

pub trait Bar:
    self::sealed::Sealed // comment1
    + Sized                // comment2
    + Eq                   // comment3
    + Hash                 // comment4
    + Debug                // comment5
    + Clone                // comment6
    + Default              // comment7
    + Serialize            // comment8
    + for<'a> Deserialize<'a> // comment9
{
    type DoubleState: Copy                                // Note(Evrey): Because Rust is drunk. 1
        + ShrAssign<u8>                       // Note(Evrey): Because Rust is drunk. 2
        + Shl<u8, Output = Self::DoubleState> // Note(Evrey): Because Rust is drunk. 3
        + BitAnd<Output = Self::DoubleState>  // Note(Evrey): Because Rust is drunk. 4
        + BitOrAssign                         // Note(Evrey): Because Rust is drunk. 5
        + Sub<Output = Self::DoubleState>     // Note(Evrey): Because Rust is drunk. 6
        + Into<u128>                          // Note(Evrey): Because Rust is drunk. 7
        + Debug                               // Note(Evrey): Because Rust is drunk. 8
        + Eq                                  // Note(Evrey): Because Rust is drunk. 9
        + Hash                                // Note(Evrey): Because Rust is drunk. 10
        + Default                             // Note(Evrey): Because Rust is drunk. 11
        + Serialize                           // Note(Evrey): Because Rust is drunk. 12
        + for<'a> Deserialize<'a>; // Note(Evrey): Because Rust is drunk. 13
}

pub trait Foo:
    self::sealed::Sealed
    + Sized
    + Eq
    + Hash
    + Debug
    + Clone
    + Default
    + Serialize
    + for<'a> Deserialize<'a>
{
    type DoubleState: Copy
        + ShrAssign<u8>
        + Shl<u8, Output = Self::DoubleState>
        + BitAnd<Output = Self::DoubleState>
        + BitOrAssign
        + Sub<Output = Self::DoubleState>
        + Into<u128>
        + Debug
        + Eq
        + Hash
        + Default
        + Serialize
        + for<'a> Deserialize<'a>;
}
