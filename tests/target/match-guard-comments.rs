/**************************************************************
 * Match pre/post Guard comments - between Condition ("if") and arrow ("=>")
 **************************************************************/

// Test 1
fn main() {
    match var.name {
        name if
        //BAD COMMENT
        name.contains("smth")
            // other comment
            | name.contains("smth else") =>
        {
            true
        }
        _ => false,
    }
}
// Test 2
fn main() {
    match var.name {
        name if name.contains("smth")
            // other comment
            | name.contains("smth else") =>
        {
            true
        }
        _ => false,
    }
}
// Test 3
fn main() {
    match name {
        Some(A) if A == '/' && B == '*' => 88,
    }
}
// Test 4
fn main() {
    match name {
        /*v*/ name if /*w*/ 5 => {
            /*y*/
            6
        } /*z*/
    }
}
// Test 5
fn main() {
    match name {
        /*v*/
        name if // w
        5 =>
        {
            /*y*/
            6
        } /*z*/
    }
}
// Test 6
fn main() {
    match name {
        /*v*/ name if /*w*/ 5 /*x*/ => {
            /*y*/
            6
        } /*z*/
    }
}
// Test 7
fn main() {
    match name {
        /*v*/
        name if // w
        5 /*x*/ =>
        {
            /*y*/
            6
        } /*z*/
    }
}
// Test 8
fn main() {
    match name {
        name if
        //BAD COMMENT
        5 =>
        {
            6
        }
    }
}
// Test 9
fn main() {
    match name {
        name if //BAD COMMENT
        5 =>
        {
            6
        }
    }
}
// Test 10
fn main() {
    match name {
        name if /*BAD COMMENT*/ 5 => 6,
    }
}
// Test 11
fn main() {
    match name {
        name if
        /*BAD COMMENT*/
        5 =>
        {
            6
        }
    }
}
// Test 12
fn main() {
    match name {
        name if //XXXXX
        5 /*BAD COMMENT*/ =>
        {
            6
        } //YYY YYY
    }
}
// Test 13
fn main() {
    match name {
        name if 5 //BAD COMMENT
         => 6,
    }
}
// Test 14
fn main() {
    if /*BAD COMMENT*/ 1 == 2 {
        7
    }
}
// Test 15
fn main() {
    match name {
        name if 5 => 6,
    }
    let i = 7; /* x */
}
