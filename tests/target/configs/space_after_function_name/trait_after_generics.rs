// rustfmt-space_after_function_name: AfterGenerics
// Trait space after function name

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
