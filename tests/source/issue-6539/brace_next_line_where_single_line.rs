// rustfmt-style_edition: 2027
// rustfmt-brace_style: AlwaysNextLine
// rustfmt-where_single_line: true

// Top-level functions

// Short function
fn short_fn(a: f64, b: f64) -> f64;

// Function with wrapping return type and no where clause
fn fn_with_long_return_type(a: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>>;

// Function with non-wrapping return type and where clause
fn fn_with_short_where<T>(a: f64, b: T) -> f64 where T: Debug;

// Function that wraps at a simple return type
fn fn_with_wrapping_return_type<T>(aaaaaa: f64, bbbbbb: T, cccccc: f64, dddddd: f64, eeee: f64) -> f64;

// Function that wraps at the where clause
fn fn_with_wrapping_where_clause<T>(aaaaaa: f64, bbbbbb: T, cccccc: f64, dddddd: f64) -> f64 where T: Debug;

// Function with both wrapping return type and wrapping where clause
fn fn_with_long_return_and_where<T, U, 'a>(a: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator;

// Function with wrapping arguments, return type, and where clause
fn fn_with_everything_long<T, U, 'a>(aaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64, ffff: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator;

// Same variations with bodies
fn short_fn_with_body(a: f64, b: f64) -> f64 {}

fn fn_with_long_return_type_and_body(a: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> {}

fn fn_with_short_where_and_body<T>(a: f64, b: T) -> f64 where T: Debug {}

fn fn_with_wrapping_return_type_and_body<T>(aaaaaa: f64, bbbbbb: T, ccc: f64, ddd: f64, ee: f64) -> f64 {}

fn fn_with_wrapping_where_clause_and_body<T>(aaaaaa: f64, bbbbbb: T, ccc: f64, ddd: f64) -> f64 where T: Debug {}

fn fn_with_long_return_and_where_and_body<T, U, 'a>(a: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator {}

fn fn_with_everything_long_and_body<T, U, 'a>(aaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64, ffff: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator {}

// Trait methods
pub trait Trait {
    fn short_method(a: f64, b: f64) -> f64;

    fn method_with_long_return(&self) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>>;

    fn method_with_short_where<T>(&self, a: f64, b: T) -> f64 where T: Debug;

    fn method_with_wrapping_return_type<T>(self, aaa: f64, bbb: T, ccc: f64, ddd: f64, ee: f64) -> f64;

    fn method_with_wrapping_where_clause<T>(aaaaaa: f64, bbbbbb: T, ccc: f64, ddd: f64) -> f64 where T: Debug;

    fn method_with_long_return_and_where<T, U, 'a>(self) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator;

    fn method_with_everything_long<T, U, 'a>(&self, aaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator;

    // Same variations with bodies
    fn short_method_with_body(a: f64, b: f64) -> f64 {}

    fn method_with_long_return_and_body(&self) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> {}

    fn method_with_short_where_and_body<T>(&self, a: f64, b: T) -> f64 where T: Debug {}

    fn method_with_wrapping_return_type_and_body<T>(aaaa: f64, bb: T, cc: f64, d: f64, e: f64) -> f64 {}

    fn method_with_wrapping_where_clause_and_body<T>(aaa: f64, bb: T, cc: f64, d: f64) -> f64 where T: Debug {}

    fn method_with_long_return_and_where_and_body<T, U, 'a>(self) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator {}

    fn method_with_everything_long_and_body<T, U, 'a>(aaaa: f64, bbbb: f64, cccc: f64, dddd: f64, eeee: f64) -> Result<HashMap<String, Vec<(SomeLongTypeName, AnotherLongTypeName, YetAnotherType)>>, Box<dyn Error + Send + Sync + 'static>> where T: Debug + Display + Clone + Send + Sync + 'static, U: Iterator<Item = &'a T> + ExactSizeIterator {}
}
