macro_rules! reproduce {
    (type Fail = $ty:ty; arr = $($arr:expr),+) => {
        ( vec![$($arr),+] )
    };
    ( $expr:expr, $($arr:item),+) => {
        1
    };
}

fn main() {
    reproduce!(type Fail = char; arr = 1);
    reproduce!(23, type Fail = char;, type Fail = char;);
}
