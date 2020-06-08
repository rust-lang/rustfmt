// rustfmt-indent_style=Visual

fn main() {
    let grammar: Vec<_> = ast
        .attrs
        .iter()
        .filter(|attr| match attr.value {
            MetaItem::NameValue(ref ident, _) => format!("{}", ident) == "grammar",
            _ => false,
        })
        .collect();
}
