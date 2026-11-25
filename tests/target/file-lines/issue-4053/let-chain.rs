//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/let-chain.rs","range":[7,7]},{"file":"tests/source/file-lines/issue-4053/let-chain.rs","range":[20,20]},{"file":"tests/source/file-lines/issue-4053/let-chain.rs","range":[35,35]}]
//@ rustfmt-edition: 2024

fn main() {
    // Selecting a `let` in the chain.
    if let Some(a) = x
        && a == 1
        && let Some(b) = y
    {
        let     _     =     1     ;
    }

    // Selecting a non-`let` condition in the chain.
    if let Some(a) = x
        && a == 1
        && let Some(b) = y
    {
        let     _     =     1     ;
    }

    // Selecting a statement in the body.
    if let Some(a) = x
        && a == 1
        && let Some(b) = y
    {
        let _ = 1;
    }

    // Nothing selected.
    if
    let  Some(a)  =  x
    &&
    a  ==  1
    &&
    let  Some(b)  =  y
    {
        let     _     =     1     ;
    }
}
