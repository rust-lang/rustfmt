fn main() {
    if let Some(1) = Some(1)
        && let Some(2) = Some(2)
        && true
        && let true = false
        && let false = true
    {
        let _x = 1 + 1;
        let _y = 2;
    }
}
