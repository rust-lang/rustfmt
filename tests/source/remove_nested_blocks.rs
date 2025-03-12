// rustfmt-remove_nested_blocks: true
fn foo() {}

// The way it is implemented right now, only 'naked' blocks are removed.
// This means unsafe blocks, const blocks, blocks with labels and attributes,
// blocks belonging to other constructs such as loops are not removed.
fn main() {
    {
        {
            {
                foo();
            }
        }
    }
    // The next block will be removed
    {
        {
            // Blocks with comments are not removed
            {
                {
                    /* second comment */
                    {
                        foo();
                    }
                }
            }
        }
    }
    {
        {
            /*
             * multi-line comment
             */
            foo();
        }
    }
    {{{{{{{foo();}}}}}}}
    {{/*comment*/{{{foo();}}}}}
    {{/*comment*/{{/*comment*/{foo();}}}}}
    {
        const { const {} }
    }
    const { const {} }
    unsafe { unsafe {} }
    // As const and unsafe blocks are not 'naked' blocks, they are not removed.
    unsafe {
        unsafe {
            {
                const {
                    const {
                        {
                            foo();
                        }
                    }
                }
            }
        }
    }
    const {
        unsafe {
            {
                unsafe {
                    const {
                        {
                            foo();
                        }
                    }
                }
            }
        }
    }
    {
        'label1: {
            'label2: {
                foo();
            }
        }
    }
    'outer: loop {
        {
            'inner: loop {
                {
                    foo();
                }
            }
        }
    }
    if let Some(x) = 5f64.map(|x| Ok(x)) {
        {
            if let Some(x) = 5f64.map(|x| Ok(x)) {
                foo();
            }
        }
    }
    if false {
        { {} }
    }
    #[cfg(debug)]
    {
        { {} }
    }
}
