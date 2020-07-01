// rustfmt-space_before_fn_params: true
// rustfmt-max_width: 118
// Trait space before function parameters

trait Story {
    fn swap_context<T: 'static + Context + Send + Sync>(&mut self, context: T) -> Option<Box<Context + Send + Sync>>;
}

impl Story for () {
    fn swap_context<T: 'static + Context + Send + Sync>(&mut self, context: T) -> Option<Box<Context + Send + Sync>> {
        // ...
    }
}
