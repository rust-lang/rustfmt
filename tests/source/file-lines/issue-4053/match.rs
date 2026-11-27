//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/match.rs","range":[7,7]},{"file":"tests/source/file-lines/issue-4053/match.rs","range":[13,13]},{"file":"tests/source/file-lines/issue-4053/match.rs","range":[24,24]}]

fn main() {
    // Selecting one arm.
    match 1 {
    1=>(),
    2 => { let _ = (); }
    3=>{     }
    }

    // Selecting the scrutinee.
    match
    1   +   1
    {
    1=>(),
    2 => { let _ = (); }
    3=>{     }
    }

    // Selecting the pattern of one arm.
    match 1 {
    1=>(),
    2 => { let _ = (); }
    3
    =>{     }
    }

    // Nothing selected.
    match
    1   +   1
    {
    1=>(),
    2 => { let _ = (); }
    3
    =>{     }
    }
}
