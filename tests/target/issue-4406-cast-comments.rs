/*****
 *  Tests for proper formatting of pre and post cast ("as") comments
 ******/

// Test 1
fn main() {
    let x = 0f64 /* x as */ as i32;
}

// Test 2
fn main() {
    let x = 1 /* foo as */ as i32;
}

// Test 3
fn main() {
    let x = 1 as /* bar as */ i32;
}

// Test 4
fn main() {
    let x = 1 /* as foo */ as /* as bar */ i32;
}

// Test 5
fn main() {
    let x = 1 /* as foo */ as /* as bar */ i32;
}

// Test 6
fn main() {
    let x = 1 /* as foo */ as /* as bar */ i32;
}

// Test 7
fn main() {
    let x = 1 /* as foo yyyyyyyyyyy */
        as /* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/ i32;
}

// Test 8
fn main() {
    let x = 1 /* as foo yyyyyyyyyyy */
        as /* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/
        i32;
}

// Test 9
fn main() {
    let x = 1 /* as foo yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy */
        as /* as bar xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx*/
        i32;
}

/*****
 *  Tests for not leaving trailing spaces related to cast comments (related to #2896?)
 ******/
// Test 10 - extra blank after the binary rhs at the 2nd line (comment followws at 3rd line)
fn main() {
    if 0 == 1 /* x */ as i32 {}
}

// Test 11 - extra blank after the binary rhs at the end of 2nd line
fn main() {
    if 0 == ' ' as i32 {}
}

// Test 12 - extra blank after the comment at the end of 2nd line
fn main() {
    if 0 == ' ' /* x */ as i32 {}
}

/*****
 *  Tests for not moving "as" to new line unnecessarily - from #3528
 ******/
fn get_old_backends(old_toml_config: &toml::Value) -> Option<Vec<Box<dyn Backend>>> {
    old_toml_config.as_table().and_then(|table| {
        table
            .get("backends")
            .and_then(|backends| backends.as_table())
            .map(|backends| {
                backends
                    .into_iter()
                    .filter_map(|(key, value)| match AvailableBackend::from(key.as_str()) {
                        AvailableBackend::Git => {
                            Some(Box::new(Git {
                                config: value.clone().try_into::<GitConfig>().unwrap(),
                            }) as Box<dyn Backend>)
                        }
                        AvailableBackend::Github => {
                            Some(Box::new(Github {
                                config: value.clone().try_into::<GithubConfig>().unwrap(),
                            }) as Box<dyn Backend>)
                        }
                    })
                    .collect()
            })
    })
}
