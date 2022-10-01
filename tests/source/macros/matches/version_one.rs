//rustfmt-version: One

// To visually verify the special case handling of `matches!` we include the equivalent match expr

// issue #4462
fn issue_4462() {
matches!(c,
    'x' | 'c' | 'b' | 'B' | '?' | 'h' | 'H' | 'i' | 'I' | 'l' | 'L' | 'q' | 'Q' | 'n'
    | 'N' | 'f' | 'd' | 's' | 'p' | 'P');

match c {
    'x' | 'c' | 'b' | 'B' | '?' | 'h' | 'H' | 'i' | 'I' | 'l' | 'L' | 'q' | 'Q' | 'n'
    | 'N' | 'f' | 'd' | 's' | 'p' | 'P' => {}
}
}

// issue #4885
fn issue_4885() {
matches!(
    c,
    '\\' | '.' | '+' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' |
    '$'  | '#' | '&' | '-' | '~' | '*' | '?'
);

match c {
    '\\' | '.' | '+' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' |
    '$'  | '#' | '&' | '-' | '~' | '*' | '?' => {}
}
}

// issue #5176
fn issue_5176() {
matches!(self, | Self::A | Self::B);
match self {
    | Self::A | Self::B => {}
}
}

// issue #5547
fn issue_5547() {
    matches!(something.very_very_very.long.even.more.fields, Some(very_long_field_name_name_to_check) if method(very_long_field_name));
    match something.very_very_very.long.even.more.fields {
        Some(very_long_field_name_name_to_check) if method(very_long_field_name) => {}
    }
}

// other scenarios
fn other_scenarios() {

    // no guard with trailing comma
    matches!(self, | Self::A | Self::B,);
    match self {
        | Self::A | Self::B => {}
    }

    // guard with trailing comma
    matches!(something.very_very_very.long.even.more.fields, Some(very_long_field_name_name_to_check) if method(very_long_field_name),);
    match something.very_very_very.long.even.more.fields {
        Some(very_long_field_name_name_to_check) if method(very_long_field_name) => {}
    }

    // short expr and pattern, but guard is long.
    matches!(something, Some(_) if method(very_long_input_1, very_long_input_2, very_long_input_3, very_long_input_4, very_long_input_5),);
    match something {
        Some(_) if method(very_long_input_1, very_long_input_2, very_long_input_3, very_long_input_4, very_long_input_5) => {}
    }

    // square brackets
    matches![self, | Self::A | Self::B];
    match self {
        | Self::A | Self::B => {}
    }
    // curly braces
    matches!{self, | Self::A | Self::B};
    match self {
        | Self::A | Self::B => {}
    }
}

// nested matches! calls
impl Mystruct {
    pub(crate) fn is_expr(&self) -> bool {
        matches!(
            self,
            OverflowableItem::Expr(..)
                | OverflowableItem::MacroArg(MacroArg::Expr(..))
                | OverflowableItem::MatchMacroItem(MatchMacroItem::Expr(..))
        );

        match self {
            OverflowableItem::Expr(..)
                | OverflowableItem::MacroArg(MacroArg::Expr(..))
                | OverflowableItem::MatchMacroItem(MatchMacroItem::Expr(..)) => {}
        }
    }

    pub(crate) fn is_expr_with_guard(&self) -> bool {
        matches!(
            self,
            OverflowableItem::Expr(..)
                | OverflowableItem::MacroArg(MacroArg::Expr(..))
                | OverflowableItem::MatchMacroItem(MatchMacroItem::Expr(..))
                if self.condition()
        );

        match self {
            OverflowableItem::Expr(..)
                | OverflowableItem::MacroArg(MacroArg::Expr(..))
                | OverflowableItem::MatchMacroItem(MatchMacroItem::Expr(..))
                if self.condition() => {}
        }
    }
}

fn multi_line_struct_pattern_with_guard() {
    matches!(
        token,
        Token::Dimension {
            value,
            ref unit,
            ..
        } if num_context.is_ok(context.parsing_mode, value)
    );

    match token {
        Token::Dimension {
            value,
            ref unit,
            ..
        } if num_context.is_ok(context.parsing_mode, value) => {
            // body
        },
    }
}
