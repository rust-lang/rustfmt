// rustfmt-version: One

// Original - return-type and empty "where" - no comments
mod inner {
fn foo1() -> String
where {
String::new()
}
}

mod inner {
fn foo2() -> String
{
String::new()
}
}

// Original from #4649
trait Foo {
    fn bar(&self)
    where
    //     Self: Bar
    // Some comment
    ;
}

fn foo<T>()
where
//     T: Bar,
// Some comment
{
    println!("foo");
}

// Return-type with no "where" - one comment
fn main () {
fn foo1() -> String /* same-line with ret-type and brace comment */ {
}
}

fn main () {
fn foo2() -> String /* same-line with ret-type comment - brace in new-line */
{
}
}

fn main () {
fn foo3() -> String
/* new-line comment - brace in new-line */
{
}
}

fn main () {
fn foo4() -> String // same-line with ret-type comment
{
}
}

// Return-type and empty "where" - one comment
fn main () {
fn foo1() -> String /* same-line with ret-type/where/brace brace pre-where comment */ where {
}
}

fn main () {
fn foo2() -> String /* same-line with ret-type/where pre-where comment - brace in new-line */ where
{
}
}

fn main () {
fn foo3() -> String /* same-line with ret-type pre-where comment - where in new-line */
where {
}
}

fn main () {
fn foo4() -> String
/* new-line pre-where comment - where in new-line */
where {
}
}

fn main () {
fn foo5() -> String // same-line with ret-type pre-where comment
where
{
}
}

fn main () {
fn foo6() -> String
// new-line pre-where comment
where
{
}
}

// Return-type and empty "where" - two inline comments
fn main () {
fn foo1() -> String /* pre-where same-line */ where /* post-where same line */ {
}
}

fn main () {
fn foo2() -> String
/* pre-where new-line */ where /* post-where same line */ {
}
}

fn main () {
fn foo3() -> String /* pre-where same with ret - where in new */
where /* post-where same line */ {
}
}

fn main () {
fn foo4() -> String /* pre-where same with ret - where in new */
where
/* post-where new line - brace in same */ {
}
}

fn main () {
fn foo5() -> String
/* pre-where new line - where in new */
where
/* post-where new line - brace in same */ {
}
}

fn main () {
fn foo6() -> String
/* pre-where new line - where in new */
where
/* post-where new line - brace in new */
{
}
}

// Return-type and empty "where" - two one-line comments
fn main () {
fn foo1() -> String // pre-where same with ret - where in new
where // post-where same with where - brace in new
{
}
}

fn main () {
fn foo2() -> String
// pre-where new line
where // post-where same with where - brace in new
{
}
}

// Return-type and empty "where" - more comments
fn main() {
fn foo<F>(foo2: F) -> String /* pre-where same with ret - where in new */
where /* post-where same with where - following in new" */
F: Fn() /* comment after where declaration */
{
}
}
