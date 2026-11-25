//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/if-else.rs","range":[6,6]},{"file":"tests/source/file-lines/issue-4053/if-else.rs","range":[27,27]},{"file":"tests/source/file-lines/issue-4053/if-else.rs","range":[47,47]},{"file":"tests/source/file-lines/issue-4053/if-else.rs","range":[61,61]}]

fn main() {
    // Selecting the `if` condition.
    if x == 1 {
        let     _     =     1     ;
    } else if x == 2 {
        let     _     =     2     ;
    } else {
        let     _     =     3     ;
    }

    // Selecting the `else if` condition.
    if x == 1 {
        let     _     =     1     ;
    } else if x == 2 {
        let     _     =     2     ;
    } else {
        let     _     =     3     ;
    }

    // Selecting the `else`.
    if x == 1 {
        let     _     =     1     ;
    } else if x == 2 {
        let     _     =     2     ;
    } else {
        let     _     =     3     ;
    }

    // Selecting a statement in the `else if` branch.
    if x == 1 {
        let     _     =     1     ;
    } else if x == 2 {
        let _ = 2;
    } else {
        let     _     =     3     ;
    }

    // Nothing selected.
    if
    x  ==  1
    {
        let     _     =     1     ;
    }
    else  if
    x  ==  2
    {
        let     _     =     2     ;
    }
    else
    {
        let     _     =     3     ;
    }
}
