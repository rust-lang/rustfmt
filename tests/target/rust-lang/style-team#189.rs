// rustfmt-style_edition: 2024

impl SomeType {
    fn method(&mut self) {
        self.array[array_index as usize]
            .as_mut()
            .expect("thing must exist")
            .extra_info = Some(ExtraInfo {
                parent,
                count: count as u16,
                children: children.into_boxed_slice(),
            });
    }
}

impl SomeType {
    fn method(&mut self) {
        self.array[array_index as usize]
            .as_mut()
            .expect("thing must exist")
            .extra_info =
                long_long_long_long_long_long_long_long_long_long_long_long_long_long_long;
    }
}
