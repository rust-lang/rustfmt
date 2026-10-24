//@ rustfmt-unstable: true
//@ rustfmt-edition: 2024
//@ rustfmt-match_block_trailing_comma: true

cfg_select! {
    _ => a,
}

cfg_select! {
    _ => {{ b }}
}

cfg_select! {
    _ => c,
}

cfg_select! {
    _ => d,
}

cfg_select! {
    _ => {{ e }},
}

// Comment before comma

cfg_select! {
    _ => f, // line comment
}

cfg_select! {
    _ => g, /* block comment */
}

cfg_select! {
    _ => {{ h }} // line comment
}

cfg_select! {
    _ => {{ i }} /* block comment */
}

cfg_select! {
    _ => j // line comment
    ,
}

cfg_select! {
    _ => k /* block comment */,
}

cfg_select! {
    _ => { l } /* block comment */,
}

cfg_select! {
    _ => {{ m }} /* block comment */,
}

// Mixed

cfg_select! {
    _ => {{ n }}
    _ => unsafe { .. },
}

cfg_select! {
    _ => {{ o }},
    _ => unsafe { .. },
}

cfg_select! {
    _ => {{ p }},
    _ => unsafe { .. },
}

// Max width boundary cases, where widths are of the source line.
// For the `*_no_comma_100_cols` double brace cases, we'd exceed the max width if we add a `,`.

cfg_select! {
    _ => single_brace_no_comma_100_cols_________________________________________________________,
}

cfg_select! {
    _ => single_brace_no_comma_101_cols__________________________________________________________,
}

cfg_select! {
    _ => {{ double_brace_no_comma_100_cols_______________________________________________________ }}
}

cfg_select! {
    _ => {{
        double_brace_no_comma_101_cols________________________________________________________
    }}
}

cfg_select! {
    _ => single_brace_with_comma_100_cols______________________________________________________,
}

cfg_select! {
    _ => single_brace_with_comma_101_cols_______________________________________________________,
}

cfg_select! {
    _ => {{ double_brace_with_comma_100_cols____________________________________________________ }},
}

cfg_select! {
    _ => {{
        double_brace_with_comma_101_cols_____________________________________________________
    }},
}

cfg_select! {
    _ => single_brace_line_comment_100_cols_____________________________________, // line comment
}

cfg_select! {
    _ => single_brace_line_comment_101_cols______________________________________, // line comment
}

cfg_select! {
    _ => single_brace_block_comment_100_cols________________________________, /* block comment */
}

cfg_select! {
    _ => single_brace_block_comment_101_cols_________________________________, /* block comment */
}

cfg_select! {
    _ => { single_brace_block_comment_then_comma_100_cols____________________ } /* block comment */,
}

cfg_select! {
    _ => { single_brace_block_comment_then_comma_101_cols_____________________ } /* block comment */,
}
