fn main() {
    let single_comment = ||
    // this should be indented.
        1;

    let block_comment = ||
        /* This is a long,
         * explanatory comment.
         * Put this all in a block.
         */
        1;

    let nested_closure = ||
    // indent + wrap in a block.
        || 1;

    let nested_comment_closure = ||
    // if you see this code in real life, run.
        ||
            // nested closures don't need blocks,
            // but comments do.
        || 1;
}
