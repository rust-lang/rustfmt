mod a {
    pub mod b {}
    pub fn b() {}
}

fn b() {}

use a::b::self;
