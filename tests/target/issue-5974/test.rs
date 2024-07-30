macro_rules! repro {
    () => {
        #[doc = concat!("let var = ",
        "false;")]
        fn f() {}
    };
}
