macro_rules! bad {
    () => {
        macro_rules! inner {
            () => {
                // This needs to have a width of over 100 characters to trigger the issue 12345678901
                ("a", "B")
            };
        }
    };
}
