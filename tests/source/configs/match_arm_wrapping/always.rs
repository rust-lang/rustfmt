// rustfmt-match_arm_wrapping: Always
// Wrap match-arms

fn main() {
    match lorem {
        1 => foooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo(x),
        2 => {
            println!("{}", sit)
        }
        3 => panic!(),
        4 => (),
        y => {
            // Some comment
            let ipsum = y - 1;
            println!("{}", ipsum);
        }
    }
}
