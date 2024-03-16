// rustfmt-wrap_comments: true

// Ensure comment preservation in parenthesis
fn main() {
    let (/* */ () | () /* */) = ();
    let (/**/ () | () /**/) = ();
    let (/*comment*/ () | () /*comment*/) = ();
    let (/*multi-line
           comment*/ () | () /*multi-line
                                comment */) = ();
    let (/*comment with new line
         */
         () | ()
         /*comment with new line
         */
        ) = ();
}

// Ensure proper handling of line comments
fn line_comments() {
    let (// Before
         () | ()
         // After
        ) = ();
}

// Ensure proper handling of block comments with new lines
fn block_comments_with_new_lines() {
    let (/* Before
          * with new line */
         () | ()
         /* After
         * with new line */
        ) = ();
}

// Ensure inner pattern is getting formatted properly
// whilst preserving comments in outer parenthesis
fn inner_pat_formatting() {
    let (/*comment*/
        () | ()
    /*comment*/) = ();
}
