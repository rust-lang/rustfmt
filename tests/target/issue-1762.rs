// rustfmt-indent_style: Visual
// rustfmt-max_width: 30

fn main() {
    let grammar: Vec<_> = ast
        .attrs
        .iter()
        .filter(|attr| match attr.value {
            MetaItem::NameValue(ref ident, _) => format!("{}", ident) == "grammar",
            _ => false,
        })
        .collect();

    let a = b.iter()
             .map(|m| m.hi());
}
