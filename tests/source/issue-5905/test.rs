macro_rules! test_macro {
    ($member:ident $($rest:tt)*) => {
        paste::paste! {fn test(&self) {
        (self.$member$($rest)* )
        }}
    };
}

