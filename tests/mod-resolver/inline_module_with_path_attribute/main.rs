#[path = "bravo"]
mod alpha {
    mod charlie;

    #[path = "echo"]
    mod delta {
        mod foxtrot;
    }

    #[path = "hotel/india"]
    mod golf {
        mod juliet;
        #[path = "lima.rs"]
        mod kilo;
    }
}

mod mike {
    mod november;
}

// Similar to issue https://github.com/rust-lang/rustfmt/issues/4076
#[path = "oscar"]
mod oscar {
    mod papa;
}

// A more extreme case of https://github.com/rust-lang/rustfmt/issues/3901
#[path = "."]
mod quebec {
    #[path = "."]
    mod romeo {
        #[path = "."]
        mod sierra {
            mod tango;
        }
    }
}
