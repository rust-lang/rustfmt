// rustfmt-format_brace_macros: true

fn issue_1917() {
    mod x {
        quickcheck! {
            fn test(a: String, s: String, b: String) -> TestResult {
                if a.find(&s).is_none() {

                    TestResult::from_bool(true)
                } else {
                    TestResult::discard()
                }
            }
        }
    }
}

fn foo() {
    f! {r#"
         test
    "#};
}

x! {()}
x! {#}
x! {ident}

macro_rules! moo1 {
    () => {
        bar! {
"
"
        }
    };
}

macro_rules! moo2 {
    () => {
        bar! {
        "
"
        }
    };
}

macro_rules! moo4 {
    () => {
        bar! {
"
    foo
        bar
baz"
        }
    };
}

macro_rules! impl_from_vector {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt | $source:ident) => {
        impl From<$source> for $id {
            #[inline]
            fn from(source: $source) -> Self {
                fn static_assert_same_number_of_lanes<T, U>()
                where
                    T: crate::sealed::Simd,
                    U: crate::sealed::Simd<LanesType = T::LanesType>,
                {
                }
                use llvm::simd_cast;
                static_assert_same_number_of_lanes::<$id, $source>();
                Simd(unsafe { simd_cast(source.0) })
            }
        }

        // FIXME: `Into::into` is not inline, but due to
        // the blanket impl in `std`, which is not
        // marked `default`, we cannot override it here with
        // specialization.
        /*
        impl Into<$id> for $source {
            #[inline]
            fn into(self) -> $id {
                unsafe { simd_cast(self) }
            }
        }
        */

        test_if! {
            $test_tt: interpolate_idents! {
                mod [$id _from_ $source] {
                    use super::*;
                    #[test]
                    fn from() {
                        assert_eq!($id::lanes(), $source::lanes());
                        let source: $source = Default::default();
                        let vec: $id = Default::default();

                        let e = $id::from(source);
                        assert_eq!(e, vec);

                        let e: $id = source.into();
                        assert_eq!(e, vec);
                    }
                }
            }
        }
    };
}

// https://github.com/rust-lang/rustfmt/issues/3434
html! { <div>
Hello
</div>}

// https://github.com/rust-lang/rustfmt/issues/3445
wrapper! {
use std::  collections::HashMap;
pub fn the (a: sdf )
{
    ( )
}
}

// https://github.com/rust-lang/rustfmt/issues/5254
widget! {
    /// A frame around content
    ///
    /// This widget provides a simple abstraction: drawing a frame around its
    /// contents.
    #[autoimpl(Deref, DerefMut on self.inner)]
    #[autoimpl(class_traits where W: trait on self.inner)]
    #[derive(Clone, Debug, Default)]
    #[handler(msg = <W as Handler>::Msg)]
    #[widget{
        layout = frame(self.inner, kas::theme::FrameStyle::Frame);
    }]
    pub struct Frame<W: Widget> {
        #[widget_core]
        core: CoreData,
        #[widget]
        pub inner: W,
    }

    // NOTE: `impl Self` is not valid Rust syntax, but rustfmt handles it fine
    impl Self {
        /// Construct a frame
        #[inline]
        pub fn new(inner: W) -> Self {
            Frame {
            core: Default::default(),
                inner,
            }
        }
    }
}

// https://users.rust-lang.org/t/rustfmt-skips-macro-arguments/74807
macro_rules! identity {
    ($x:expr) => {
        $x
    };
}

fn main() {
    foo("first very very long argument", "second very very long argument");
    identity! { foo("first very very long argument", "second very very long argument") };
}

fn main() {
    foo("first very very long argument", "second very very long argument");
    identity! { foo("first very very long argument", "second very very long argument", ) };
}

#[tokio::main]
fn main() {
    // This gets formatted as expected
    foo(
    "first very very long argument",
        "second very very long argument",
    );
    tokio::select! (
        _ = futures::future::ready(true) => {
            // This is left unchanged
            foo("first very very long argument", "second very very long argument")
        }
    )
}

// https://github.com/rust-lang/rustfmt/issues/4611
// Note that this is not currently formatted.
tokio::select! {
    result = reader => {
        match result {
            Ok(v) => {
                eprintln!(
                "message: {}",
                v
                );
            },
            Err(_) => {
                eprintln!(
                    "error: {}",
                    e
                );
            },
        }
    },
    _ = writer => {
        // Comment
        eprintln!(
            "completed: {}",
            some_other_field
        );
    }
}
