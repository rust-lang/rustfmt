// rustfmt-reorder_type_constraints: true

fn foo<T: 'static + Clone + Copy + Debug + PartialEq>(a: T) {
    todo!();
}
