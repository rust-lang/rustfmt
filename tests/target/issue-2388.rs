macro lex_err($kind: ident $(, $body: expr)*) {
    Err(QlError::LexError(LexError::$kind($($body,)*)))
}
