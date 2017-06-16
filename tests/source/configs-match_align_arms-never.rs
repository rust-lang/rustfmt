// rustfmt-match_align_arms: Never
// Align match arms

fn main() {
    match lorem {
        Lorem::Ipsum => (),
        Lorem::Dolor => (),
        Lorem::Sit => (),
        Lorem::Amet => (),
    }

    match lorem {
        Lorem::Ipsum => (),
        Lorem::Dolor => (),
        Lorem::Sit   => (),
        Lorem::Amet  => (),
    }

    match lorem {
        Lorem::Ipsum    => (),
        Lorem::Dolor    => (),
        Lorem::Sit      => (),
        Lorem::Amet     => (),
    }
}
