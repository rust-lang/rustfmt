//@ rustfmt-style_edition: 2027
//@ rustfmt-max_width: 50

#[requires(
x < y &&
x > 0
)]
#[requires({
    let x = x + 1;
    let y = y + 1;
x < y &&
x > 0
}
)]
#[requires({
    let x = x + 1;
    let y = y + 1;
// line comment within
x < y &&
x > 0
}
)]
#[requires
        // line comment start
({
    let x = x + 1;
    let y = y + 1;
x < y &&
x > 0
}
)]

  // line comment betweeen



#[requires ({
    let x = x + 1;
    let y = y + 1;
x < y &&
x > 0
}
)
  // line comment end
]

#[requires ({
    let x = x + 1;
    let y = y + 1;
  /* block comment within */
x < y &&
x > 0
}
)
]
#[requires ({
    let x = x + 1;
    let y = y + 1;
		INVALID EXPR INVALID EXPR
x < y &&
x > 0
}
)
]

#[unsafe   (   requires ({
	let x = x + 1;
	let y = y + 1;
		x < y &&
		x > 0
}
))
]

#[   requires (   arg0 * 2, arg1 == a + b + c
)
]

#[   requires (1 + trailing_comma,
)
]
#[   requires (*looooooooooooooong_expr, 1 + trailing_comma_multiline,
)
]

#[   requires [ 1 +   * brackets
]
]
#[   requires { 1 +   * braces
}
]

#[requires(*this_attr_is_49_chars_wide_________)]
#[requires(*this_attr_is_50_chars_wide__________)]
#[requires(*this_attr_is_51_chars_wide___________)]
fn function_with_contract(x: u32, y: u32) {}

fn main() {}
