// rustfmt-float_literal_trailing_zero: Never

fn float_literals() {
    let a = 0.;
    let b = 0.0;
    let c = 100.;
    let d = 100.0;
    let e = 5e3;
    let f = 5.0e3;
    let g = 7f32;
    let h = 7.0f32;
    let i = 9e3f32;
    let j = 9.0e3f32;
    let k = 1000.00;
    let l = 1_000_.;
    let m = 1_000_.000_000;
}

fn line_wrapping() {
    let array = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        17.0, 18.0,
    ];
    println!(
        "This is floaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat {}",
        10.0e3
    );
}
