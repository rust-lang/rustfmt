// rustfmt-version: Two

// Original issue - parameters in macro call

macro_rules! outer {
    ($d:tt) => {
        macro_rules! inner {
            ($d s:expr) => {
                println!("{}", $d s);
            }
        }
    };
}

outer!($);

fn main() {
    inner!("hi");
}

macro_rules! outer {
    ($d:tt) => {
        macro_rules! inner {
            ($d s:expr) => {
                println!("{}", $d    s);
            }
        }
    };
}

// Variations of the original issue

macro_rules! outer {
($d:tt) => {
macro_rules! inner {
($d s:expr) => {
println!("{}", $d s);
}
}
};
}

macro_rules! outer {
($d:tt) => {
macro_rules! inner {
($d s:expr) => {
println!("{}", $d    s);
}
}
};
}

fn main() {
macro_rules! outer {
($ d:tt) => {
macro_rules! inner {
($ d s:expr) => {
println!("INNER1: {}", $d s);
println!("INNER2: {}", $ s);
}
}
};
}

outer!($);

inner!("hi");
}
    
// Consecutive identities in macro body

fn main() {
macro_rules! uniop {
($op:tt, $s:expr) => {
$op $s
};
}
let x = uniop!(!, true);
println!("{}", x);
let x = uniop!(-, 7);
println!("{}", x);
}

fn main() {
macro_rules! binop {
($l:expr, $op:tt, $r:expr) => {
$l             $op                $r
};
}
let x = binop!(10, -, 7);
println!("{}", x);
let x = binop!(10, +, 7);
println!("{}", x);
}
