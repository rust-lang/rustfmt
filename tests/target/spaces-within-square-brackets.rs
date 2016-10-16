// rustfmt-spaces_within_square_brackets: true

fn main() {

    let arr: [ i32; 5 ] = [ 1, 2, 3, 4, 5 ];
    let arr: [ i32; 500 ] = [ 0; 500 ];

    let v = vec![ 1, 2, 3 ];
    assert_eq!(arr, [ 1, 2, 3 ]);

    let i = arr[ 0 ];

    let slice = &arr[ 1..2 ];
}

fn f(slice: &[ i32 ]) {}
