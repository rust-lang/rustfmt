// rustfmt-fn_generics_space: OnlyBefore
// rustfmt-max_width: 118
// Spacing around trait function generics

trait Story {
    fn swap_context <T: 'static + Context + Send + Sync>(&mut self, context: T)
    -> Option<Box<Context + Send + Sync>>;
}

impl Story for () {
    fn swap_context <T: 'static + Context + Send + Sync>(
        &mut self,
        context: T,
    ) -> Option<Box<Context + Send + Sync>> {
        // ...
    }
}
