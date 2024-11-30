use std::cmp::Ordering;
use print๙msg::print as first_print;
use print0msg::print as second_print;
use printémsg::print as third_print;

fn main() {
    first_print();
    second_print();
    third_print();

    assert_eq!("print๙msg".cmp("printémsg"), Ordering::Greater);
}

/// '๙' = 0E59;THAI DIGIT NINE;Nd;
mod print๙msg {
    pub fn print() {
        println!("Non-ASCII Decimal_Number")
    }
}

/// '0' = 0030;DIGIT ZERO;Nd;
mod print0msg {
    pub fn print() {
        println!("ASCII Decimal_Number")
    }
}

/// 'é' = 00E9;LATIN SMALL LETTER E WITH ACUTE;Ll;
mod printémsg {
    pub fn print() {
        println!("Lowercase_Letter")
    }
}