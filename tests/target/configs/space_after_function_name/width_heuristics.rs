// rustfmt-space_after_function_name: AfterGenerics
// rustfmt-max_width: 118
// Trait space after function name

trait Story {
    fn swap_context<T: 'static + Context + Send + Sync> (&mut self, context: T)
    -> Option<Box<Context + Send + Sync>>;
}

impl Story for () {
    fn swap_context<T: 'static + Context + Send + Sync> (
        &mut self,
        context: T,
    ) -> Option<Box<Context + Send + Sync>> {
        // ...
    }
}
