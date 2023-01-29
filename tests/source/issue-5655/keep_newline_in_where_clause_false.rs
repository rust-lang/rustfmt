// keep_newline_in_where_clause: false
fn foo<T>(_: T)
where
    T: std::fmt::Debug,
    T: std::fmt::Display,
{
}
