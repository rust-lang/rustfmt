fn func<F: Fn(usize) -> usize>(f: F) -> usize {
    f(0)
}

fn main() {
    let _result = func(|x| 
        match x {
            _ => 0,
        } + match 1 {
            _ => 1,
        }
    );
}
