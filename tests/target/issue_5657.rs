// rustfmt-max_width: 80

fn omg() {
    fn bar() {
        self.core
            .exchange::<
                LeaseBufReader<_, BUFSIZ>,
                _,
                LeaseBufWriter<_, BUFSIZ>,
                _,
            >(device_index, src, dest)
            .map_err(RequestError::from)
    }
}
