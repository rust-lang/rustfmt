// Based on issue #2055:
pub trait A {}
pub trait B {}
pub trait C {}
pub trait Foo1: A + C + B {}
pub trait Foo2:
    // A and C
    A + C + B
{
}
pub trait Foo3:
    /* A and C */
    A + C + B
{
}
pub trait Foo4: // A and C
    A + C + B
{
}
pub trait Foo5: /* A and C */ A + C + B {}
pub trait Foo6: /* A and C */ A + C + B {}

// Other cases
trait Person {
    fn name(&self) -> String;
}
/*comment1*/
trait Person {
    fn name(&self) -> String;
}
trait Student: /* comment1 */ Person /* comment2 */ {
    fn university(&self) -> String;
}
trait Programmer /* comment1 */ {
    fn fav_language(&self) -> String;
}
trait CompSciStudent1: /* comment1 */ Programmer + Student /* comment2 */ {
    fn git_username(&self) -> String;
}
trait CompSciStudent2:
    /* comment1 Longggggggggggggggggggggggggggggggggggggggggggggggggg */
    Programmer + Student /* comment2 */
{
    fn git_username(&self) -> String;
}
trait CompSciStudent3: // comment1
    Programmer + Student /* comment2 */
{
    fn git_username(&self) -> String;
}
trait CompSciStudent4: // comment1 Longgggggggggggggggggggggggggggggggggggggggggggggggggg
    Programmer + Student /* comment2 */
{
    fn git_username(&self) -> String;
}

// Comment before Ident
trait /* comment1 */ Person {
    fn fav_language(&self) -> String;
}
trait // comment1
Person {
    fn fav_language(&self) -> String;
}
trait /* comment 1 */ Programmer /* comment2 */ {
    fn fav_language(&self) -> String;
}
trait /* comment1 */ CompSciStudent1: /* comment2 */ Programmer + Student /* comment3 */ {
    fn git_username(&self) -> String;
}
