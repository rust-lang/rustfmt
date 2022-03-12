// rustfmt-wrap_comments: true

/// [MyType](VeryLongPathToMyType::NoLineBreak::Here::Okay::ThatWouldBeNice::Thanks)
fn documented_with_longtype() {
    // # We're using a markdown header in an inline comment. rustfmt should be
    // able to wrap this comment when `wrap_comments = true`
}
