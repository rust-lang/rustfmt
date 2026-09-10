fn main() {
    // case 1: fits on one line
    x = X { field1, field2, xx };
    let X { field1, field2, .. } = x;

    // case 2: literal is too wide (vertical), pattern should be single-line
    x = X { field1, field2x, xx };
    let X { field1, field2x, .. } = x;

    // case 3: both literal and pattern are too wide — both vertical
    x = X { field1, field2, field3, xx };
    let X { field1, field2, field3, .. } = x;
}
