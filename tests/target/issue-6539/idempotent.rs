// rustfmt-style_edition: 2027

// Function with the maximum length before wrapping (100 characters)
fn a_one_hundred_column_fn_decl(aaaaaaaaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64) -> f64;

fn a_one_hundred_column_fn_decl_with_body(aaaaa: f64, bbb: f64, cc: f64, dd: f64, ee: f64) -> f64 {}

fn a_ninety_nine_column_fn_decl(aaaaaaaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64) -> f64;

fn a_ninety_nine_column_fn_decl_with_body(aaaaa: f64, bb: f64, cc: f64, dd: f64, ee: f64) -> f64 {}

fn a_one_hundred_one_column_fn_decl(
    aaaaaaaaaaa: f64,
    bbb: f64,
    ccc: f64,
    ddd: f64,
    eee: f64,
) -> f64;

fn a_one_hundred_one_column_fn_decl_with_body(aaaaaa: f64, bb: f64, c: f64, d: f64, e: f64) -> f64 {
}

pub trait Trait {
    fn a_one_hundred_column_fn_decl(&self, aaaaaaa: f64, bb: f64, cc: f64, dd: f64, ee: f64) -> f64;

    fn a_one_hundred_column_fn_decl_with_body(&self, aaaa: f64, bb: f64, cc: f64, dd: f64) -> f64 {}

    fn a_ninety_nine_column_fn_decl(&self, aaaaaa: f64, bb: f64, cc: f64, dd: f64, ee: f64) -> f64;

    fn a_ninety_nine_column_fn_decl_with_body(&self, aaa: f64, bb: f64, cc: f64, dd: f64) -> f64 {}

    fn a_one_hundred_one_column_fn_decl(
        &self,
        aaaaaaaa: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
    ) -> f64;

    fn a_one_hundred_one_column_fn_decl_with_body(
        &self,
        aaaaa: f64,
        b: f64,
        c: f64,
        d: f64,
    ) -> f64 {
    }
}
