//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/closure.rs","range":[8,8]},{"file":"tests/source/file-lines/issue-4053/closure.rs","range":[21,21]},{"file":"tests/source/file-lines/issue-4053/closure.rs","range":[28,28]}]

fn main() {
    // Selecting the closure signature.
    let f = move |a: u32, b: u32| {
        let     c     =     a     ;
        let     d     =     b     ;
        c     +     d
    };

    // Selecting a statement in the closure body.
    let f = move |a: u32, b: u32| {
        let c = a;
        let     d     =     b     ;
        c     +     d
    };

    // Selecting the variable being assigned.
    let f = move |a: u32, b: u32| {
        let     c     =     a     ;
        let     d     =     b     ;
        c     +     d
    };

    // Nothing selected.
    let
    f
    =
    move  |  a  :  u32  ,  b  :  u32  |
    {
        let     c     =     a     ;
        let     d     =     b     ;
        c     +     d
    };
}
