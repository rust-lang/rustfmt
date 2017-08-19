// rustfmt-treat_match_arm_bodies_as_statements: false
// Option to format match arm bodies like freestanding statements

fn main() {
    match lorem {
        Ipsum(dolor) => if dolor { 0 } else { 1 },
    }
}
