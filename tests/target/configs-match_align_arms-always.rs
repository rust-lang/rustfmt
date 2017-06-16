// rustfmt-match_align_arms: Always
// Align match arms

fn main() {
    match lorem {
        Lorem::Ipsum => (),
        Lorem::Dolor => (),
        Lorem::Sit   => (),
        Lorem::Amet  => (),
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

    match lorem {
        Lorem::Ipsum => (),
        Lorem::Dolor => {
            lorem();
            ipsum();
        }
        Lorem::Sit   => {
            lorem();
            ipsum();
        }
        Lorem::Amet  => (),
    }
}
