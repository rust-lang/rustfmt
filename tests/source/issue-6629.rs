macro_rules! reproduce {
    (type Fail = $ty:ty; arr = $($arr:expr),*) => {
        ( vec![$($arr),+] )
    };
}

fn main() {
    reproduce!(type Fail = char; arr = 1);
}
