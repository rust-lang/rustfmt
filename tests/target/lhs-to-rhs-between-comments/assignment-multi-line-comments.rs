// rustfmt-version: Two
// "=" Assignemnt - Multiline Block comments
fn main() {
    let var =
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        first;
    var =
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        second;
    var =
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2
         * Block comment line 3 */
        third;
    var =
        /* Block comment line 1
         * Block comment line 2 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 3 */
        forth;
}

// "=" Assignemnt - Multiline Line and Block comments
fn main() {
    let var = // Line comment 1
        // Line comment 2
        // Line comment 3
        first;
    var = // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        second;
    var = /* Block comment single line */
        // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        third;
    var = /* Block comment single line 1 */
        /* Block comment single line 2*/
        forth;
    var = /* Block comment single line 1 */
        /* Block comment single line 2*/
        fifth;
    var =
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2 */
        // Line comment
        sixth;
    var = // Line comment
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2 */
        seventh;
}

// ": type =" Assignemnt - Multiline Line and Block comments
fn main() {
    let var: int =
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        first;
    let var: u64 =
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        second;
    let var: f32 = // Line comment 1
        // Line comment 2
        // Line comment 3
        third;
    let var: bool = // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        forth;
    let var: char = /* Block comment single line */
        // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        fifth;
    let var: str = /* Block comment single line 1 */
        /* Block comment single line 2*/
        fsixth;
    let var: usize = /* Block comment single line 1 */
        /* Block comment single line 2*/
        seventh;
}

// BinOp Assignemnt - Multiline Block comments
fn main() {
    let var =
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        first;
    var +=
        /* Block comment line 1
         * Block comment line 2
         * Block comment line 3 */
        second;
    var -=
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2
         * Block comment line 3 */
        third;
    var *=
        /* Block comment line 1
         * Block comment line 2 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 3 */
        forth;
}

// BinOp Assignemnt - Multiline Line and Block comments
fn main() {
    let var = // Line comment 1
        // Line comment 2
        // Line comment 3
        first;
    var += // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        second;
    var -= /* Block comment single line */
        // Line comment
        /* Block comment line 1
         * Block comment line 2 */
        third;
    var *=
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2 */
        // Line comment
        forth;
    var /= // Line comment
        /* Block comment line 1 longggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
         * Block comment line 2 */
        fifth;
}
