//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/match.rs","range":[7,7]}]

fn main() {
    match 1 {
        1 => (),
        2 => {
let _ = ();
        }
    }
}
