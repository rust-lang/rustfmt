// `r#` opens a raw string only when the run of `#`s ends in a `"`. A raw identifier
// such as `r#struct` must stay classified as normal code, or the macro body rewriter
// copies part of it through verbatim and puts the `$` back in the wrong place.

#![feature(decl_macro)]

macro_rules! r#struct {
        ($r#struct:expr)  =>  {  $r#struct  }
}

macro r#decl_macro($r#fn:expr) {
        $r#fn
}

macro_rules! old_macro {
    ($a:expr) => {
        $a
    };
}

fn raw_ident_next_to_a_raw_string() {
    let r#match = r#"still a raw string"#;
    let r#fn = r##"hashes too"##;
    let _ = (r#match, r#fn);
}
