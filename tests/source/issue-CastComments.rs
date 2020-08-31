/* Tests proper formatting of pre and post cast ("as") comments */

// Test 1
fn main() {
    let x = 0f64            /* x as */ as i32;
}

// Test 2
fn main() {
let x = 1       /* foo as */   as      i32;
}

// Test 3
fn main() {
let x = 1                  as        /* bar as */  i32;
}

// Test 4
fn main() {
let x = 1       /* as foo */   as        /* as bar */  i32;
}

// Test 5
fn main() {
let x = 1       /* as foo */as/* as bar */  i32;
}

// Test 6
fn main() {
let x = 1       /* as foo */
as/* as bar */
i32;
}

// Test 7
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/  i32;
}

// Test 8
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/  i32;
}

// Test 9
fn main() {
let x = 1       /* as foo yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy */
as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/
i32;
}
