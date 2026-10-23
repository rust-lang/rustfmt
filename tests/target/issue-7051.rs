// 1. comment is exactly 100 columns
use std
/* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */
;

// 2. line with `;` is exactly 100 columns
use std /* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

// 3. line with `;` is 101 columns
use std
/* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

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

mod x {

    use bar // foo
    ;
    use foo /* a */ /* b */;

    mod x {
        use bar // foo
        ;
    }

    fn f() {
        use baz // foo
        ;
    }

    // 1. comment is exactly 100 columns
    use std
    /* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */
    ;

    // 2. line with `;` is exactly 100 columns
    use std /* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

    // 3. line with `;` is 101 columns
    use std
    /* aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

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
}

use bar // foo
;
use foo /* a */ /* b */;

mod x {
    use bar // foo
    ;
}

fn f() {
    use baz // foo
    ;
}
