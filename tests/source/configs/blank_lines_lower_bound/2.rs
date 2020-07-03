// rustfmt-blank_lines_lower_bound: 2
// rustfmt-blank_lines_upper_bound: 3

#[foo]
fn foo() {
    println!("a");
}
#[bar]
#[barbar]
fn bar() {
    println!("b");
    println!("c");
}
struct Foo {}
enum Bar {}
use std::io;
extern crate foobar;
extern crate foo;
extern crate bar;
trait Foo = Bar;
impl Foo {}
mac!();
#[temp]
use std::fs;
use std::alloc;
use std::ascii;
