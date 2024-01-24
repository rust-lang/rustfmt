// rustfmt-single_line_simple_if: true
// rustfmt-unstable_features: true

fn main() {
    // if statements may be formatted on a single line if they are "short"
    // and only contain a single expression of 'return', 'continue' or 'break'
    if true { continue }

    if true {
        continue
    }
    
    // Default max width is 50
    if width == 50_characters_or_shorter { continue }
    if width == 51_characters_long_and_above { return }

    if name == super_duper_really_really_mega_ultra_giga_long_name_with_a_cherry_on_top { return }
    
    // More than 1 stmt means a new line, it is no longer 'simple'
    if a { let y = 1; return y; }
    
    // Adds a semicolon to 'return/continue/break' when put on a new line 
    // (unless config has trailing_semicolon = false)
    if a { let y = 1; return y }
    
    // Will not work on fn or method calls (may change)
    if true { do_something() }

    // Will not work on an expression with trailing semicolon pre-format 
    if true { return; }

    // Will not single line if there is an else block, even with single expressions
    if true { return } else { break }

    // Will not be single line if returns/breaks with a value
    for i in 0..2{
        if true { break }
        if true { break 2 }
        if true { return }
        if true { return 3 }
    }

    // Will not be single line if comment is in the block
    if true {
        // nope
        return
    }
    if true { /* nope 2 */ return }

    // Only works on if blocks, not other control flow
    for i in 0..2 { if i == 1 { continue } }

    for i in 0..2 {
        loop { if i == 1 { continue } }
    }

    // New line formatted here as 'loop' != 'return/continue/break'
    if i == 1 { loop { return } }
    
    // Works on labelled break/continue
    'gamer: loop { if true{ break 'gamer } }

    'gamer: loop { if true{ break 'gamer; } }

    let result = 'block: {
        if foo() { break 'block 1 }
        if bar() { break 'block 2; }
        3
    };

    #[allow(unused)]
    // Comments after attributes dont mess it up
    if true { return }
    #[cfg(target_os = "linux")]
    // Comments after attributes dont mess it up
    if name == super_duper_ultra_really_name { return }
    #[cfg(target_os = "linux")]
    /* Multiple lines dont mess this up */
    /* Multiple lines dont mess this up */
    if name == super_duper_ultra_really_name { return }

    // Works as intended with nested ifs and indents
    if true {
        if true { continue }
        if true { if true { continue } }
    } else if false {        
        if true { if true { if width == 50_characters_or_shorter { continue } if width == 51_characters_long_and_above { return } } }
    } else {
        if true { return; }
    }

    // Works with complex conditions
    if matches!(x, Ok(Some(value))) { continue }
    if matches!(x, Ok(Some(value))) { kick_ball() }
    if matches!(x, Ok(Some(value))) && value.some_method_call(input) { break }
    if matches!(x, Ok(Some(value))) && value.some_method_call(input) { run_fast() }
    if matches!(x, Ok(Some(value))) && value.some_method_call(input) && single_line_if_is_allowed_at_all_ever { return }
    if matches!(x, Ok(Some(value))) && value.some_method_call(input) && single_line_if_is_allowed_at_all_ever { play_catch() }

    // Nested complex conditions
    if true {
        if matches!(x, Ok(Some(value))) { continue }
        if true { if matches!(x, Ok(Some(value))) && value.some_method_call(input) { break } }
    } else if false {        
        if true { if true { if matches!(x, Ok(Some(value))) { continue } } }
    } else {
        if true { if true { if matches!(x, Ok(Some(value))) && value.some_method_call(input) && single_line_if_is_allowed_at_all_ever { return } } }
    }
}