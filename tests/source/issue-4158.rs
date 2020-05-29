// rustfmt-wrap_comments: true
// rustfmt-max_width: 50

// This is a long line that will be wrapped on the next line.
// This line will be wrapped with it.
// ```
// this is code that won't
// even if it also is very very very very very very very very very very very very long
// ```
// This is a second long line that will be wrapped on the next line.
// ```
// this is code that won't
// ```

/// This is a long line that will be wrapped on the next line.
/// ```
/// Should handle code blocks with no end
fn outer() {
    //! This is a long line that will be wrapped on the next line.
    //! ```rust
    //! assert!(true);
    //! ```
    fn inner() {
        /* This is a long line that will be wrapped on the next line.
         * ```rust
         * assert!(true);
         * ```
         */
    }
}
