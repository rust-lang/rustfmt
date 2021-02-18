#![allow(dead_code)] // bar

//! Doc comment
fn test() {
// comment
        // comment2

    code(); /* leave this comment alone!
             * ok? */

        /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a
         * diam lectus. Sed sit amet ipsum mauris. Maecenas congue ligula ac quam
         * viverra nec consectetur ante hendrerit. Donec et mollis dolor.
         * Praesent et diam eget libero egestas mattis sit amet vitae augue. Nam
         * tincidunt congue enim, ut porta lorem lacinia consectetur. Donec ut
         * libero sed arcu vehicula ultricies a non tortor. Lorem ipsum dolor sit
         * amet, consectetur adipiscing elit. Aenean ut gravida lorem. Ut turpis
         * felis, pulvinar a semper sed, adipiscing id dolor. */

    // Very loooooooooooooooooooooooooooooooooooooooooooooooooooooooong comment that should be split

                    // println!("{:?}", rewrite_comment(subslice,
                    //                                       false,
                    //                                       comment_width,
                    //                                       self.block_indent,
                    //                                       self.config)
                    //                           .unwrap());

    funk(); //dontchangeme
            // or me
}

  /// test123
fn doc_comment() {
}

/*
Regression test for issue #956

(some very important text)
*/

/*
fn debug_function() {
    println!("hello");
}
// */

#[link_section=".vectors"]
#[no_mangle] // Test this attribute is preserved.
#[cfg_attr(rustfmt, rustfmt::skip)]
pub static ISSUE_1284: [i32; 16] = [];

// issue 4668
fn f() {
    fn g() -> int {
        let foo = 12;
        match foo {
            0..=10 => {
                /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin urna felis, molestie ex. */
            }
            _ => {
                let st = None;
                let res = st.unwrap_or(
                    /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin urna felis, molestie ex. */ "abc"
                );
            }
        }
    }
}

fn h() {
    let mut y = 0;
    while let x = vec![1, 2, 3]
        .iter()
        /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin urna felis, molestie ex... */
        .take(3)
        .skip(1)
        .to_owned()
        .next()
    {
        y += x.unwrap_or_else(|/* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin urna felis, molestie ex... */_| &0) * 2;
    }
}

pub fn t(x: int) {
    if x == 3 {
        let y = match context {
            True => {
                |x| /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin urna felis, molestie ex... */ x+2
            }
            False => |x| x,
        };
    } else {}
}
