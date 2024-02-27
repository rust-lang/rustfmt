// rustfmt-float_literal_trailing_zero: Never

fn float_literals() {
    let a = 0.;
    let b = 0.;
    let c = 100.;
    let d = 100.;
    let e = 5e3;
    let f = 5e3;
    let g = 7f32;
    let h = 7f32;
    let i = 9e3f32;
    let j = 9e3f32;
    let k = 1000.;
    let l = 1_000_.;
    let m = 1_000_.;
}

fn line_wrapping() {
    let array = [
        1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.,
    ];
    println!("This is floaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat {}", 10e3);
}
