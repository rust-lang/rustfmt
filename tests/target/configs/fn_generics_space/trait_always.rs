// rustfmt-fn_generics_space: BeforeAndAfter
// rustfmt-fn_no_generics_space: true
// Trait space after function name

trait Story {
    fn swap_context <T> (&mut self, context: T) -> Option<Box<Context>>
    where
        T: Context;
}

impl Story {
    fn swap_context <T> (&mut self, context: T) -> Option<Box<Context>>
    where
        T: Context,
    {
        // ...
    }
}
