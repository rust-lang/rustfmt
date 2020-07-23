// rustfmt-space_before_fn_sig_paren: true
// Trait space before function paren

trait Story {
    fn swap_context<T> (&mut self, context: T) -> Option<Box<Context>>
    where
        T: Context;
}

impl Story {
    fn swap_context<T> (&mut self, context: T) -> Option<Box<Context>>
    where
        T: Context,
    {
        // ...
    }
}
