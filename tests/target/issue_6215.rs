cfg_if!(
    if #[cfg(feature = "xxxx")] {
        if #[cfg(feature = "yyyy")] {
            pub const C1: u8 = 1;
        } else {
            pub const C2: u8 = 2;
        }
    }
);
