fn main() {
    let requires_multiline = 7;

    let _ = {
        || if true {
            requires_multiline
        } else {
            requires_multiline
        }..19;
    };
}
