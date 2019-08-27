fn main() {
    let Test {
        #[cfg(feature = "test")]
        x,
    } = Test {
        #[cfg(feature = "test")]
        x: 1,
    };

    let Test {
        #[cfg(feature = "test")]
        // comment
        x,
    } = Test {
        #[cfg(feature = "test")]
        x: 1,
    };
}
