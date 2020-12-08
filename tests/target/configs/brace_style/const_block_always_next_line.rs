// rustfmt-brace_style: AlwaysNextLine
// AlwaysNextLine brace style for const blocks

fn foo() -> i32
{
    const
    {
        let x = 5 + 10;
        x / 3
    }
}

fn bar() -> i32
{
    const { 4 }
}

fn foo() -> i32
{
    const
    {
        let x = 5 + 10;
        x / 3
    }
}

fn foo() -> i32
{
    const // baz
    {
        let x = 5 + 10;
        x / 3
    }
}

fn foo() -> i32
{
    const /*qux */
    {
        let x = 5 + 10;
        x / 3
    }
}

fn foo() -> i32
{
    const
    // baz
    {
        let x = 5 + 10;
        x / 3
    }
}
