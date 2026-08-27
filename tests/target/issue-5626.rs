// run-pass

pub fn main() {
    return isize();
}

pub mod foo {
    pub fn f() -> isize {
        return 2;
    }
    pub fn g() {
        || || || || || || || {
            H || || {
                isize
                    || || || || || || || {
                        (|| || || {
                            isize
                                || || || || || || || || {
                                    isize || || || || || || || || || || || || || isize!(f(), 2)
                                }
                        })()
                    }
            }
        };
        isize!(::f(), 1);
    }
}

pub fn main() {
    return || || || || || || || {
        H || || {
            isize
                || || || || || || || {
                    (|| || || {
                        H || || || || || || || || {
                            isize || || || || || || || || || || || || || isize!(f(), 2)
                        }
                    })()
                }
        }
    };
}
