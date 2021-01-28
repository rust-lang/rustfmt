// Original issue
macro_rules! outer {
    ($d:tt) => {
		macro_rules! inner {
			($d s:expr) => {
				println!("{}", $d s);
			}
		}
	};
}

// Other tests with illegal code
fn main() {
    let s = 7;

    macro_rules! outer {
        ($d:tt) => {
			macro_rules! inner {
				($d s:something) => {
					println!("{}", $d s);
				}
			}
		};
    }

    outer!(5);
    inner!(5 s:something);
}
macro_rules! outer {
    () => {
		macro_rules! inner {
			() => {
				println!("", $dddddd sssssss);
			}
		}
	};
}

// Tests with legal code
macro_rules! outer {
    () => {
        macro_rules! inner {
            () => {
                println!("", zddddd, ssssss, vvvvvvv);
            };
        }
    };
}
macro_rules! outer {
    ($d:tt) => {
        macro_rules! inner {
            ($d s:expr) => {};
        }
    };
}
fn main() {
    let s = 7;

    macro_rules! outer {
        ($d:tt) => {
            macro_rules! inner {
                ($d s:something) => {
                    println!("{}{}", $d, s);
                };
            }
        };
    }

    outer!(5);
    inner!(5 s:something);
}
