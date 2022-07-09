// rustfmt-spaces_within_parenthesized_items: true
#[cfg( test )]
fn foo1() {}

fn foo1_a(/* comment */) {}

fn foo1_b(/* comment */ /* comment */) {}

fn foo2( arg1: i32 ) -> i32 {
    0
}

fn foo2_a( /* comment */ arg1: i32 ) -> i32 {
    0
}

fn foo2_b( arg1: i32 /*comment */ ) -> i32 {
    0
}

fn foo3<T, U>( arg1: T, arg2: U ) -> T {
    Dummy( 1 );
    let ( x, y ) = ( 1, 2 );
    ( /* comment */ 1, 2 );
    ( 1, /* comment */ 2 );
    ( 1, 2 /* comment */ );
    0
}

fn foo3_a<T, U>( arg1: T, /* comment */ arg2: U ) -> T {
    0
}

fn foo4(
    arggggggggggggggggggggggggggggggggggggggg1: i32,
    arggggggggggggggggggggggggggggggggggggggg2: i32,
) {
}

fn foo4_a(
    /* comment */
    arggggggggggggggggggggggggggggggggggggggg1: i32,
    arggggggggggggggggggggggggggggggggggggggg2: i32,
) {
}

fn foo5() -> i32 {
    foo1();
    foo1(/* comment */);
    foo2( 1 );
    foo2( /* comment */ 1 );
    foo2( 1 /* comment */ );
    foo3( 1, 2 );
    foo3( 1, /* comment */ 2 );
    foo4(
        000000000000000000000001111111111,
        000000000000000000000002222222222,
    );
    foo2( 000000000000000000000000000000000000000000000000000000000000000000000000000000000000001 );
    foo2(
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001,
    );
    foo2(
        0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001,
    );
    println!( "{}", "hello" );
    vec![1, 2, 3, 4].iter().contains( |x| x == 1 );
    (1 * ((2 + 1) * 3))
}

type Bar = fn( i32, i32 ) -> ();

macro_rules! add {
    ($a:expr,$b:expr) => {{
        $a + $b
    }};
}
