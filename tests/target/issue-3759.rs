fn main() {
    let Test {
        #[cfg(feature = "test")]
        x,
    } = Test {
        #[cfg(feature = "test")]
        x: 1,
    };
}
