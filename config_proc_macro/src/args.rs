use syn::{
    parenthesized,
    parse::{Error as ParserError, Parse, ParseStream},
    Ident, Token,
};

#[derive(Debug, Default)]
pub struct Args {
    pub skip_derive: Option<Vec<String>>,
}

impl Args {
    pub fn skip_derives(&self) -> impl Iterator<Item = &str> + '_ {
        self.skip_derive
            .as_ref()
            .into_iter()
            .flatten()
            .map(|s| s.as_ref())
    }
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self, ParserError> {
        let mut args = Self::default();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "skip_derive" => {
                    let content;
                    parenthesized!(content in input);
                    args.skip_derive = Some(
                        content
                            .parse_terminated(Ident::parse, Token![,])?
                            .into_iter()
                            .map(|i| i.to_string())
                            .collect(),
                    );
                }
                a => panic!("unknown attribute: {}", a),
            }
        }

        Ok(args)
    }
}
