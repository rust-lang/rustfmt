// rustfmt-skip_macro_names: ["items"]

macro_rules! items {
    ($($arg:item)*) => { $($arg)* };
}

// Should skip this invocation
items!(
        const _: u8 = 0;
);

// Should not skip this invocation
use self::items as renamed_items;
renamed_items!(
    const _: u8 = 0;
);
