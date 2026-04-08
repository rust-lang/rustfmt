// Test that Unicode Pattern_White_Space characters are not stripped after line continuation

fn main() {
    let str = "who is olaf\
\u{00A0}This is a rust code example to show a bug in Unicode Pattern WhiteSpace";
}
