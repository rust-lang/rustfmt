// rustfmt-file_lines: [{"file":"tests/source/issue-4053/match.rs","range":[7,7]}]

fn main() {
    match 1 {
        1 => (),
        2 => {
            let _ = ();
        }
    }
}
