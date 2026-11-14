//@ rustfmt-format_macro_bodies: true
//@ rustfmt-chain_width: 72

macro_rules! inside_macro {
    () => {
        let value = aaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbb().cccccccccccccccc().ddddddddddddd();
    };
}

fn outside_macro() {
    if true {
        let value = aaaaaaaaaaaaaaaa.bbbbbbbbbbbbbbbb().cccccccccccccccc().ddddddddddddd();
    }
}
