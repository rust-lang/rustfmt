// rustfmt-brace_style: AlwaysNextLine
// AlwaysNextLine brace style for unsafe blocks

fn main()
{
    unsafe
    {
        let good = ();
    }

    unsafe {}

    unsafe
    {
        let ugly = ();
    }

    unsafe /* f   */
    {
    }

    unsafe /* f*/
    {
        let x = 1;
    }

    unsafe
    { /*lol*/
    }
}
