macro_rules! __diesel_operator_to_sql {
    (
         notation = infix,
         operator_expr = $op:expr,
         field_exprs = ($left:expr, $right:expr),
     ) => {
        $left;
        $op;
        $right;
    };

    (
         notation = postfix,
         operator_expr = $op:expr,
         field_exprs = ($expr:expr),
     ) => {
        $expr;
        $op;
    };

    (
         notation = prefix,
         operator_expr = $op:expr,
         field_exprs = ($expr:expr),
     ) => {
        $op;
        $expr;
    };

    ($name:ident, $operator:expr, backend: $backend:ty) => {
        diesel_postfix_operator!($name, $operator, $crate::sql_types::Bool, backend: $backend);
    };

    ($name:ident, $operator:expr, $return_ty:ty, backend: $backend:ty) => {
        __diesel_operator_body!(notation = postfix, struct_name = $name,)
    };

    ($name:ident, $operator:expr, backend: $backend:ty) => {
        diesel_prefix_operator!($name, $operator, $crate::sql_types::Bool, backend: $backend);
    };
}
