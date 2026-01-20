// rustfmt-space_after_colon: true

struct SomeStruct {
    field1: ::some_crate::Thing,
    field2 : ::some_crate::Thing,

    field3:some_crate::Thing,
    field4 :some_crate::Thing,
    field5: some_crate::Thing,
    field6 : some_crate::Thing,

    field7:i32,
    field8 :i32,
    field9: i32,
    field10 : i32,

    field11:&::some_crate::Thing,
    field12: &::some_crate::Thing,
    field13 :&::some_crate::Thing,
    field14 : &::some_crate::Thing,
}

const THING1: ::some_crate::SomeType = ::some_crate::SomeType::default();
const THING2 : ::some_crate::SomeType = ::some_crate::SomeType::default();

const THING3: some_crate::SomeType = some_crate::SomeType::default();
const THING4 :some_crate::SomeType = some_crate::SomeType::default();
const THING5: some_crate::SomeType = some_crate::SomeType::default();
const THING6 : some_crate::SomeType = some_crate::SomeType::default();

const THING7: i32 = 0;
const THING8 :i32 = 0;
const THING9: i32 = 0;
const THING10 : i32 = 0;

const THING11:&::some_crate::SomeType = ::some_crate::SomeType::default();
const THING12: &::some_crate::SomeType = ::some_crate::SomeType::default();
const THING13 :&::some_crate::SomeType = ::some_crate::SomeType::default();
const THING14 : &::some_crate::SomeType = ::some_crate::SomeType::default();



static STATIC1: ::some_crate::SomeType = ::some_crate::SomeType::default();
static STATIC2 : ::some_crate::SomeType = ::some_crate::SomeType::default();

static STATIC3: some_crate::SomeType = some_crate::SomeType::default();
static STATIC4 :some_crate::SomeType = some_crate::SomeType::default();
static STATIC5: some_crate::SomeType = some_crate::SomeType::default();
static STATIC6 : some_crate::SomeType = some_crate::SomeType::default();

static STATIC7: i32 = 0;
static STATIC8 :i32 = 0;
static STATIC9: i32 = 0;
static STATIC10 : i32 = 0;

static STATIC11:&::some_crate::SomeType = ::some_crate::SomeType::default();
static STATIC12: &::some_crate::SomeType = ::some_crate::SomeType::default();
static STATIC13 :&::some_crate::SomeType = ::some_crate::SomeType::default();
static STATIC14 : &::some_crate::SomeType = ::some_crate::SomeType::default();

fn main() {
    let x1: ::some_crate::SomeType = ::some_crate::SomeType::default();
    let x2 : ::some_crate::SomeType = ::some_crate::SomeType::default();

    let x3:some_crate::SomeType = ::some_crate::SomeType::default();
    let x4 : some_crate::SomeType = ::some_crate::SomeType::default();
    let x5: some_crate::SomeType = ::some_crate::SomeType::default();
    let x6 : some_crate::SomeType = ::some_crate::SomeType::default();

    let x7: i32 = 0;
    let x8 :i32 = 0;
    let x9: i32 = 0;
    let x10 : i32 = 0;

    let x11:&::some_crate::SomeType = ::some_crate::SomeType::default();
    let x12 :&::some_crate::SomeType = ::some_crate::SomeType::default();
    let x13: &::some_crate::SomeType = ::some_crate::SomeType::default();
    let x14 : &::some_crate::SomeType = ::some_crate::SomeType::default();


    let y_call = SomeStruct {
        field1: ::some_crate::Thing::default(),
        field2 : ::some_crate::Thing::default(),

        field3:some_crate::Thing::default(),
        field4 :some_crate::Thing::default(),
        field5: some_crate::Thing::default(),
        field6 : some_crate::Thing::default(),

        field7:12,
        field8 :12,
        field9: 12,
        field10 : 12,

        field11:&::some_crate::Thing::default(),
        field12: &::some_crate::Thing::default(),
        field13 :&::some_crate::Thing::default(),
        field14 : &::some_crate::Thing::default(),
    };

    let y_method_call = SomeStruct {
        field1: ::some_crate::Thing::Default.call(),
        field2 : ::some_crate::Thing::Default.call(),

        ..y_call
    };

    let y_binary = SomeStruct {
        field1: ::some_crate::Thing::Default+ 12,
        field2 : ::some_crate::Thing::Default + 12,

        ..y_call
    };

    let y_cast = SomeStruct {
        field1: ::some_crate::Thing::Default as i32,
        field2 : ::some_crate::Thing::Default as i32,

        ..y_call
    };

    let y_type = SomeStruct {
        field7: ::some_crate::Thing::Default,
        field8 : ::some_crate::Thing::Default,

        ..y_call
    };

    let y_field = SomeStruct {
        field1: ::some_crate::Thing::Default.some_field,
        field2 : ::some_crate::Thing::Default.some_field,

        ..y_call
    };

    let y_index = SomeStruct {
        field1: ::some_crate::Thing::Default[0],
        field2 : ::some_crate::Thing::Default[0],

        ..y_call
    };

    let y_range = SomeStruct {
        field1: ::some_crate::Thing::DefaultStart..12,
        field2 : ::some_crate::Thing::DefaultStart..12,

        ..y_call
    };

    let y_path = SomeStruct {
        field1: ::some_crate::Thing::Default,
        field2 : ::some_crate::Thing::Default,

        ..y_call
    };

    let y_mac_call = SomeStruct {
        field1: ::some_crate::macr!(),
        field2 : ::some_crate::macr!(),

        ..y_call
    };

    let y_struct = SomeStruct {
        field1: ::some_crate::Thing::SomeStruct{
            fieldA1: 123,
            fieldA2: 123,
        },
        field2 : ::some_crate::Thing::SomeStruct{
            fieldA1: 123,
            fieldA2: 123,
        },

        ..y_call
    };
}

fn func1(x: ::some_crate::SomeType) {}
fn func2(x : ::some_crate::SomeType) {}
fn func3(x:some_crate::SomeType) {}
fn func4(x :some_crate::SomeType) {}
fn func5(x: some_crate::SomeType) {}
fn func6(x : some_crate::SomeType) {}
fn func7(x:i32) {}
fn func8(x: i32) {}
fn func9(x :i32) {}
fn func10(x : i32) {}
fn func11(x:&::some_crate::SomeType) {}
fn func12(x :&::some_crate::SomeType) {}
fn func13(x: &::some_crate::SomeType) {}
fn func14(x : &::some_crate::SomeType) {}

fn print_gen_with_where1<T>(item: T)
where
    T: ::some_crate::SomeTrait + Clone,
{
    println!("{}", item.to_string());
}

fn print_gen_with_where2<T: ::some_crate::SomeTrait + Clone>(item: T) {
    println!("{}", item.to_string());
}

fn print_gen_with_where3<T: some_crate::SomeTrait + Clone>(item: T) {
    println!("{}", item.to_string());
}
