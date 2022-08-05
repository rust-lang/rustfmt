fn main() {
    let single_comment = || {
        // this should be indented.
        1
    };

    let block_comment = || {
        /* This is a long,
         * explanatory comment.
         * Put this all in a block.
         */
        1
    };

    let nested_closure = || {
        // indent + wrap in a block.
        || 1
    };

    let nested_comment_closure = || {
        // if you see this code in real life, run.
        || {
            // nested closures don't need blocks,
            // but comments do.
            || 1
        }
    };

    let attrb = || {
        // This block has an attr.
        #[foo]
        1
    };

    let after_attr = || {
        // There's a comment before...
        #[attr]
        // and one after!
        1
    };

    let one_line = || {
        /* one line */
        1
    };

    let one_comment_line = || {
        // put this comment in body
        1
    };
}
