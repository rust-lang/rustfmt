// rustfmt-wrap_comments: true
// rustfmt-comment_width: 72
// rustfmt-absolute_comment_width: true
impl MyStruct {
    fn my_fun() {
        if foo {
            // Please wrap this comment at column number 72 exactly, rather than column 84
        }

        fn bar() {
            //! Please wrap this comment at column number 72 exactly, rather than column 84
        }
    }
}
