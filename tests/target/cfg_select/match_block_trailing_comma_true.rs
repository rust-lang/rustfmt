//@ rustfmt-unstable: true
//@ rustfmt-edition: 2024
//@ rustfmt-match_block_trailing_comma: true

cfg_select! {
    _ => x,
}

cfg_select! {
    _ => {{ x }}
}

cfg_select! {
    _ => x,
}

cfg_select! {
    _ => x,
}

cfg_select! {
    _ => {{ x }},
}

// Comment before comma

cfg_select! {
    _ => x, // line comment
}

cfg_select! {
    _ => x, /* block comment */
}

cfg_select! {
    _ => {{ x }} // line comment
}

cfg_select! {
    _ => {{ x }} /* block comment */
}

cfg_select! {
    _ => x // line comment
    ,
}

cfg_select! {
    _ => x /* block comment */,
}

cfg_select! {
    _ => { x } /* block comment */,
}

cfg_select! {
    _ => {{ x }} /* block comment */,
}

// Mixed

cfg_select! {
    _ => {{ x }}
    _ => unsafe { .. },
}

cfg_select! {
    _ => {{ x }},
    _ => unsafe { .. },
}

cfg_select! {
    _ => {{ x }},
    _ => unsafe { .. },
}

// Max width boundary cases, the `,` if added would exceed default max width.

cfg_select! {
    _ => xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
}

cfg_select! {
    _ => yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,
}

cfg_select! {
    _ => {{ xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx }}
}

cfg_select! {
    _ => {{
        yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
    }}
}

cfg_select! {
    _ => xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
}

cfg_select! {
    _ => yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,
}

cfg_select! {
    _ => {{
        yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
    }},
}

cfg_select! {
    _ => xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx, // line comment
}

cfg_select! {
    _ => yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy, /* block comment */
}

cfg_select! {
    _ => xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx, // line comment
}

cfg_select! {
    _ => { yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy } /* block comment */,
}
