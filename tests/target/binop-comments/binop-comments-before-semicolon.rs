/**********************************************************************
 * Comment after BinOp rhs but before the ";".
 *
 * Notes:
 * 1. Formatting assumes no change in comment place, and requies adding the ';'
 *    by `format_stmt()` instead of `Rewrite for ast::Local()`.
 * 2. A simpler implementation may be to move the comment after the ";"
 *    (and then the above change is not required).
 * 3. It is assumed that the ";" can be appended to the end of the line even if this will cause
 *    exceeding maximum line width.  This si to make sure the ';'  will not be in a line by itself.
 ***********************************************************************/

/***********************************************
 *  No comments before the ';'
 **********************************************/
// Test 1 - no comments
fn main() {
    let x = 2 * 4;
}
// Test 2 - no comments
fn main() {
    let x = 2 * 4;
}
// Test 3 - close-comment after the ';'
fn main() {
    let x = 2 * 4; /* u */
}
// Test 4 - line-comment after the ';'
fn main() {
    let x = 2 * 4; // u
}
// Test 5 - comment after the ';' with following expression
fn main() {
    let x = 2 * 4; /* u */
    let y = 3 + 5;
}

/***********************************************
 *  One comment before the ';'
 **********************************************/
// Test 11 - closed comment before the ';'
fn main() {
    let x = 2 * 4 /* z */;
}
// Test 12 - closed comment before the ';'
fn main() {
    let x = 2 * 4 /* z */;
    let y = 3 + 5;
}
// Test 13 - line-comment comment before the ';'
fn main() {
    let x = 2 * 4 // z
    ;
}
// Test 14 - line-comment comment before the ';' with following expression
fn main() {
    let x = 2 * 4 // z
    ;
    let y = 3 + 5;
}
// Test 15 - long closed-comment before the ';'
fn main() {
    let x = 2 * 4
    /* zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz */;
}

/***********************************************
 *  Multiple comments in expression
 **********************************************/
// Test 21 - several closed comments in expression
fn main() {
    let x = 2
        * /* x */
    /* y */
        4 /* z */;
}
// Test 22 - several closed comments in expression
fn main() {
    let x = 2 /* v */    /* w */ * /* x */    /* y */ 4 /* z */; /*u*/
}
// Test 23 - several closed comments in expression
fn main() {
    let x = 2 /* v */
    /* w */
        * /* x */
    /* y */
        4 /* z */;
}
// Test 24 - several closed comments in expression with following expression
fn main() {
    let x = 2 /* v */
    /* w */
        * /* x */
    /* y */
        4 /* z */; /* u */
    let y = 3 + 5;
}
