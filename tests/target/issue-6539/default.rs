// rustfmt-style_edition: 2027

pub trait Trait {
    fn a_one_hundred_column_fn_decl_no_body(&self, aaa: f64, b: f64, c: f64, d: f64, e: f64) -> f64;

    fn a_one_hundred_one_column_fn_decl_no_body(
        self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
    ) -> f64;

    fn an_over_one_hundred_column_fn_decl_no_body_and_where_clause<T>(&self, a: T, bb: f64) -> f64
    where
        T: Debug;

    fn an_over_one_hundred_column_fn_decl_no_body_and_where_clause2<T>(
        &self,
        aaa: T,
        bbb: f64,
    ) -> f64
    where
        T: Debug;

    fn an_over_one_hundred_column_fn_decl_with_body(
        &self,
        aaaaa: f64,
        bbbbb: f64,
        ccc: f64,
    ) -> f64 {
    }

    fn an_over_one_hundred_column_fn_decl_with_body_and_where_clause<T>(&self, aaaaa: f64) -> f64
    where
        T: Debug,
    {
    }
}
