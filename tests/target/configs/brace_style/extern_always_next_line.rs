// rustfmt-brace_style: AlwaysNextLine
// AlwaysNextLine brace style for extern blocks

fn main()
{
    extern "C"
    {
        fn i_am_good(x: i32);
    }

    extern "Rust"
    {
        fn foo(x: i32);
    }

    extern "C"
    {
        #[link_name = "actual_symbol_name"]
        fn foobar();
    }

    extern "cdecl" {}

    extern "stdcall" {}

    extern "win64"
    {
        fn bar(x: i32);
    }

    extern "aapcs" {}

    extern "fastcall"
    {
        /* f*/
    }

    unsafe extern "C++"
    {
        fn lorem();
    }
}
