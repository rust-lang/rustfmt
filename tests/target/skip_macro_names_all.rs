// rustfmt-skip_macro_invocations: ["*","items"]

macro_rules! items {
    ($($arg:item)*) => { $($arg)* };
}

// Should skip this invocation
items!(
        const _: u8 = 0;
);

// Should also skip this invocation, as the wildcard covers it
use self::items as renamed_items;
renamed_items!(
        const _: u8 = 0;
);
