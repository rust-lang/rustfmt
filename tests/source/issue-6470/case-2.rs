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
}

const THING1: ::some_crate::SomeType = ::some_crate::SomeType::default();
const THING2: ::some_crate::SomeType = ::some_crate::SomeType::default();

const THING3: some_crate::SomeType = some_crate::SomeType::default();
const THING4 :some_crate::SomeType = some_crate::SomeType::default();
const THING5: some_crate::SomeType = some_crate::SomeType::default();
const THING6 : some_crate::SomeType = some_crate::SomeType::default();

const THING7: i32 = 0;
const THING8 :i32 = 0;
const THING9: i32 = 0;
const THING10 : i32 = 0;

fn main() {
    let x1: ::some_crate::SomeType = ::some_crate::SomeType::default();
    let x2: ::some_crate::SomeType = ::some_crate::SomeType::default();

    let x3:some_crate::SomeType = ::some_crate::SomeType::default();
    let x4 : some_crate::SomeType = ::some_crate::SomeType::default();
    let x5: some_crate::SomeType = ::some_crate::SomeType::default();
    let x6 : some_crate::SomeType = ::some_crate::SomeType::default();

    let x7: i32 = 0;
    let x8 :i32 = 0;
    let x9: i32 = 0;
    let x10 : i32 = 0;
}
