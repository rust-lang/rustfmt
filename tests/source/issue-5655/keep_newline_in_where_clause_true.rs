// keep_newline_in_where_clause: true
fn foo<T>(_: T)
where
    T: std::fmt::Debug,

    T: std::fmt::Display,
{
}
