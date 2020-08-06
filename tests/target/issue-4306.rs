// rustfmt-max_width: 80

trait GetMetadata {
    fn metadata(loc: ApiEndpointParameterLocation)
        -> Vec<ApiEndpointParameter>;
}
