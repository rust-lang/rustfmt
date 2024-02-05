// rustfmt-version: Two

pub struct SomeCallback(
    pub extern "C" fn(
        long_argument_name_to_avoid_wrap: u32,
        second_long_argument_name: u32,
        third_long_argument_name: u32,
    ),
);
