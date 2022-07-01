// rustfmt-match_arm_wrapping: Always
// Wrap match-arms

fn main() {
    match lorem {
        1000 => foooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo(x),
        2000 => {
            println!("{}", sit)
        }
        3000 => panic!(),
        4000 => {
            ()
        }
        5000 => this.a_very_long_function_name(foo, bar, bazz, fizz, another_argument, some_more_arguments, which_dont_fit),
        ipsum => {
            // Some comment
            let dolor = ipsum % 2;
            println!("{}", dolor);
        }
    }
}
