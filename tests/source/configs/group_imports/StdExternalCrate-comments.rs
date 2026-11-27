//@ rustfmt-group_imports: StdExternalCrate

use external_z::Z;
use std::fmt::Debug;
use external_a::A;

// standard imports
use std::io::Read;
use external_b::B;

// external imports
use crate::z::Z;
use crate::a::A;

// local imports
use crate::b::B;
