// rustfmt-reorder_type_constraints: true

fn foo<T: PartialEq + Copy + Debug + 'static + Clone>(a: T) {
    todo!();
}
