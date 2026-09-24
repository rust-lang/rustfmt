// Basic tests (using vectors) - list multiline post-comment is indented on its own
fn main() {
    let v = [
        "A",            /* item A comment */
		"BBB",    /* item B comment */
        "CCCCCC",      /* item C comment line 1
                   * item C comment line 2 */
        "D",  /* item D comment line 1
                   * item D comment line 2 */
        "EEEEE",                /* item E comment */
		"FFF", /* item F comment */
        "GG",            /* item G comment line 1
                * item G comment line 2 */
    ];
}

fn main() {
    let v = [
        "GG",            /* item G comment line 1
                * item G comment line 2 */
        "AAAAA",            /* item A comment */
		"BBB",    /* item B comment */
        "CCCCCC",      /* item C comment line 1
                   * item C comment line 2 */
        "D",  /* item D comment line 1
                   * item D comment line 2 */
        "E",                /* item E comment */
		"FFF", /* item F comment */
    ];
}

// Tests for Struct
pub(crate) struct ListFormatting<'a> {
    tactic: DefinitiveListTactic,
    separator: &'a str, /* Comment */
    trailing_separator: SeparatorTactic, /* Comment */
    separator_place: SeparatorPlace, // Comment 1
    // Comment 2
    shape: Shape,                          /* Non-expressions, e.g., items, will have a new line at the end of the list.
    * Important for comment styles. */
    ends_with_newline: bool,             // Remove newlines between list elements for expressions.
    preserve_newline: bool,                // Nested import lists get some special handling for the "Mixed" list type
    nested: bool,
    // Whether comments should be visually aligned.
    align_comments: bool, /* comment */
    config: &'a Config,
}

fn main() {
    l = ListItem {
    pre_comment,        /* Comment */
    pre_comment_style, /* Multiline comment
            * Line 2 */
    /* New line comment */
    item: if self.inner.peek().is_none() && self.leave_last { /* Comment */
        None
    } else {
        (self.get_item_string)(&item)
    },
    post_comment,       /* Comment */
    new_lines,  /* Comment */
    }
}

// Test for Function parameters
pub(crate) fn new(shape: Shape, config: &'a Config) -> Self {
    ListFormatting {
        tactic: DefinitiveListTactic::Vertical,           /* Comment */
        separator: ",",                /* comment */
        trailing_separator: SeparatorTactic::Never,                /* Multiline comment
            * second comment line */
        separator_place: SeparatorPlace::Back, // A longggggggggggggggggggggggggggggggggggggggggggggggggggg comment
        shape,
        /* New line comment */
        ends_with_newline: true,                     /* Comment */
        preserve_newline: false,        /* Comment */
        nested: false, /* Comment */
        align_comments: true,       /* Another Multiline comment
            * second comment line */
        config,         /* Last comment */
    }
}

// Test for `where`
impl<'a, T, I, F1, F2, F3> Iterator for ListItems<'a, I, F1, F2, F3>
where
    I: Iterator<Item = T>,              /* Comment */
    F111111111: Fn(&T) -> BytePos,      /* Comment */
    F2222222: Fn(&T) -> BytePos,          /* Multiline comment
                * Line 2 */
    F3: Fn(&T) -> Option<String>,   /* Comment */
{}

// Test for some types of lists
pub(crate) fn itemize_list<'a, T, I, F1, F2, F3>(
    snippet_provider: &'a SnippetProvider,      /* Comment */
    inner: I,           /* Comment */
    terminator: &'a str,        /* Multiline comment
        * Line 2 */
    separator: &'a str,     /* Comment */
    get_lo: F1,     /* Comment */
    get_hi: F2,
    get_item_string: F3,    /* Comment */
    prev_span_end: BytePos, /* Multiline comment
                                        * Line 2 */
    next_span_start: BytePos,   /* Comment */
    leave_last: bool,
) -> ListItems<'a, I, F1, F2, F3>
where
I: Iterator<Item = T>,              /* Comment */
F111111111: Fn(&T) -> BytePos,      /* Multiline comment
            * Line 2 */
F2222222: Fn(&T) -> BytePos,          
F3: Fn(&T) -> Option<String>,   /* Comment */
{
    ListItems {/* Comment to ListItems */
        snippet_provider,           /* Multiline comment
            * Line 2 */
        inner: inner.peekable(),                /* Another multiline comment
                    * another Line 2 */
        get_lo,     /* Comment */
        get_hi,
        get_item_string,
        prev_span_end,      /* Comment */
        next_span_start,/* Comment */
        terminator,
        separator,  /* Yet another multiline comment
            * yet another Line 2 */
        leave_last,/* Comment */
    }
}


// Tests when comment in the same line of the item will exceed line width
fn main() {
    let v = [
        "A-Longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",    /* item A comment */
		"BBB",    /* item B Longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg comment */
        "CCCCCC",      /* item C comment line 1
                   * item C comment line 2 */
        "D-Longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",  /* item D comment line 1
                   * item D comment line 2 */
        "EEEEE",                /* item E comment */
    ];
}

// Test for nested items
fn main() {
    let v1 = [
        "GG",            /* item G comment line 1
                * item G comment line 2 */
        "AAAAA",            /* item A comment */
            [
                "BBB",    /* item B comment */
                "CCCCCC",      /* item C comment line 1
                        * item C comment line 2 */
                "D",  /* item D comment line 1
                        * item D comment line 2 */
                "E",                /* item E comment */
            ],
		"FFF", /* item F comment */
    ];
}
