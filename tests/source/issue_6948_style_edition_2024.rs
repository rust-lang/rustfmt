//@ rustfmt-style_edition: 2024
pub fn layer<L>(self, layer: L) -> Self
where
    L: Layer<
        Route,
        Service: Service<
            Request,
            Response: IntoResponse + 'static,
            Error: Into<Infallible> + 'static,
            Future: Send + 'static,
        > + Clone + Send + Sync + 'static,
    > + Clone + Send + Sync + 'static,
{
    map_inner!(self, this => RouterInner {
        path_router: this.path_router.layer(layer.clone()),
        default_fallback: this.default_fallback,
        catch_all_fallback: this.catch_all_fallback.map(|route| route.layer(layer)),
    })
}