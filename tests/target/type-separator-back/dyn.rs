// rustfmt-type_separator: Back

fn myfunction1() -> Box<
    dyn DoubleEndedIterator<Item = (usize, usize)> +
        Send +
        Sync +
        Unpin +
        UnwindSafe +
        RefUnwindSafe,
> {
    unimplemented!();
}

fn myfunction2() -> Box<
    DoubleEndedIterator<Item = (usize, usize)> +
        Send +
        Send +
        Sync +
        Unpin +
        UnwindSafe +
        RefUnwindSafe,
> {
    unimplemented!();
}
