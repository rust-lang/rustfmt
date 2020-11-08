/*****
 *  Tests for proper formatting of pre and post cast ("as") comments.
 *  Current test don't include multi-line comments as they are not properly handled yet.
 ******/

// Test 1 - pre-cast comment
fn main() {
    let x = 0f64            /* x as */ as i32;
}

// Test 2 - pre-cast comment
fn main() {
let x = 1       /* foo as */   as      i32;
}

// Test 3 - post-cast comment
fn main() {
let x = 1                  as        /* bar as */  i32;
}

// Test 4 - pre&post-cast comment
fn main() {
let x = 1       /* as foo */   as        /* as bar */  i32;
}

// Test 5 - pre&post-cast comment
fn main() {
let x = 1       /* as foo */as/* as bar */  i32;
}

// Test 6 - pre&post-cast comment, cast in new line
fn main() {
let x = 1       /* as foo */
as/* as bar */
i32;
}

// Test 7 - pre&post-cast long comment
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/  i32;
}

// Test 8 - pre&post-cast long comment
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/  i32;
}

// Test 9 - pre&post-cast long comment, cast in new line
fn main() {
let x = 1       /* as foo yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy */
as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/
i32;
}


/*****
 *  Tests for not leaving trailing spaces related to cast comments (related to #2896?)
 ******/
// Test 10 - extra blank after the binary rhs at the 2nd line (comment followws at 3rd line)
fn main() {
    if 0 == 1 
    /* x */ as i32 {} }

// Test 11 - extra blank after the binary rhs at the end of 2nd line
fn main() {
    if 0 == ' ' 
    as i32 {} }

// Test 12 - extra blank after the comment at the end of 2nd line
fn main() {
    if 0 == ' ' /* x */ 
    as i32 {} }


/*****
 *  Test for not moving "as" to new line unnecessarily - from #3528
 ******/
 // Test 15
fn get_old_backends(old_toml_config: &toml::Value) -> Option<Vec<Box<dyn Backend>>> {
    old_toml_config.as_table().and_then(|table| {
        table
            .get("backends")
            .and_then(|backends| backends.as_table())
            .map(|backends| {
                backends
                    .into_iter()
                    .filter_map(|(key, value)| match AvailableBackend::from(key.as_str()) {
                        AvailableBackend::Git => Some(Box::new(Git {
                            config: value.clone().try_into::<GitConfig>().unwrap(),
                        })
                            as Box<dyn Backend>),
                        AvailableBackend::Github => Some(Box::new(Github {
                            config: value.clone().try_into::<GithubConfig>().unwrap(),
                        })
                            as Box<dyn Backend>),
                    })
                    .collect()
            })
    })
}


/*****
 *  Tests for for onle-line open comments ("//")
 ******/
 // Test 20
fn main() {
    if 1234 == 0
    /*xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/ & /*yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy*/ cc & dd {} }
	
// Test 21
fn main() {
    if 'a' == b // x
    as char {}
}

// Test 22
fn main() {
    if 'a' == b as  // x
	char {}
}

/*********************************
 *  Tests for multi-line comments.
 *  For FUTURE_USE when will be supported by rusfmt.
 ******
// Test 32 - Multiline pre-cast comment
fn main() {
let x = 1       /* foo as
                    * second comment line */
   as      i32;
}

// Test 33 - Multiline post-cast comment
fn main() {
let x = 1                  as        /* bar as
                * second comment line */  i32;
}

// Test 34 - Multiline pre&post-cast comment
fn main() {
let x = 1       /* foo as
* second comment line */   as        /* bar as
                * second comment line */  i32;
}

// Test 35 - Multiline pre&post-cast comment
fn main() {
let x = 1       /* as foo 
* second comment line */as/* bar as
* second comment line */  i32;
}

// Test 36 - Multiline pre&post-cast comment, cast in new line
fn main() {
let x = 1       /* as foo 
* second comment line */
as/* as bar 
* second comment line */
i32;
}

// Test 37 - Multiline pre&post-cast long comment
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx 
* second comment line */  i32;
}

// Test 38 - Multiline pre&post-cast long comment
fn main() {
let x = 1       /* as foo yyyyyyyyyyy */as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx 
* second comment line */  i32;
}

// Test 39 - Multiline pre&post-cast long comment, cast in new line
fn main() {
let x = 1       /* as foo yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy 
* second comment line */
as/* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx 
* second comment line */
i32;
}
*************** FUTURE-USE ******************************/
