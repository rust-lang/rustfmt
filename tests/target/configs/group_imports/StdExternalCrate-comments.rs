//@ rustfmt-group_imports: StdExternalCrate

use std::fmt::Debug;

use external_a::A;
use external_z::Z;

// standard imports
use std::io::Read;

use external_b::B;

// external imports
use crate::a::A;
use crate::z::Z;

// local imports
use crate::b::B;
