// rustfmt-version: Two
// "=" Assignemnt
fn main() {
    let var = first;
    var = second;
}
fn main() {
    let var = /* Block comment */ first;
    var = /* Block comment */ second;
    var = /* Block comment */ third;
}
fn main() {
    let var = // Line comment
        first;
    var = // Line comment
        second;
}
fn main() {
    let var = /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    var = /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}
fn main() {
    let var =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    var =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}

// ": type =" Assignemnt
fn main() {
    let var: u8 = first;
    var = second;
}
fn main() {
    let var: int = /* Block comment */ first;
    let var: u32 = /* Block comment */ second;
    let var: f64 = /* Block comment */ third;
}
fn main() {
    let var: bool = // Line comment
        first;
    let var: char = // Line comment
        second;
}
fn main() {
    let var: str =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    let var: usize =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}
fn main() {
    let var: isize =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    let var: u8 =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}

// BinOp Assigment
fn main() {
    let var = first;
    var += second;
}
fn main() {
    let var = /* Block comment */ first;
    var -= /* Block comment */ second;
}
fn main() {
    let var = // Line comment
        first;
    var *= // Line comment
        second;
}
fn main() {
    let var = /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    var /= /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}
fn main() {
    let var =
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        first;
    var /=
        /* Block comment longgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg */
        second;
}
