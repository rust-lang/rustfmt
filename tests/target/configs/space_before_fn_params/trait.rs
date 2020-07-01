// rustfmt-space_before_fn_params: true
// Trait space before function parameters

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
