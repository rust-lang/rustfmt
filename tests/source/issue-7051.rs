use std /* goodbye */;

use std // bye
;

use std /* comment; with semicolon */;

use std /*
    multiline
    comment
*/;

use std // some
// multi-line
// set of single comments
;

pub use std /* public import */;

use std::{
    sync::Arc /* some comment */,
    thread,
};

use std /* block comment */ // line comment
;

use std // line comment
/* block comment */;

use std // comment ending in */
;

use std // first
// second */
;

use std /* hello aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

use std /* hello aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa */;

use std::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    HashMap,
    HashSet,
    LinkedList,
    TryReserveError,
    VecDeque,
} /* comment */;

use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicPtr, AtomicUsize, Ordering} /* this comment is long enough to push the line past the 100 column limit */;

mod x {
    use std // comment
    ;

    use std /* comment */ ;

    use std // comment
    // comment
    ;

    use std /*
        comment
        comment
    */;
}
