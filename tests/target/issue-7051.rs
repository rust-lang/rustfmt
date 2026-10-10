use std /* goodbye */;

use fs; // bye

use core /* goodbye */; // bye

use std // comment */
;

use std
// comment
;

use std
/* comment */;

use alloc;
use core /* c */;
use fs; /* b */
use std /* a */;

fn main() {
    use std /* in fn */;
}

use std // some
// multi-line
// set of single comments
;

use std /* some
set of single comments */;

use std
/*
 This is a
 multi-line comment
*/;

use std::{sync::Arc /* comment */, thread};

use std::{sync::Arc, thread} // comment
;

use std::{
    sync::Arc, // comment
    thread,
};
