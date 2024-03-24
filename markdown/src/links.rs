use super::MarkdownFormatter;
use pulldown_cmark::escape::StrWrite;
use std::borrow::Cow;

impl<'i, F> MarkdownFormatter<'i, F> {
    pub(super) fn write_inline_link<S: AsRef<str>>(
        &mut self,
        url: &str,
        title: Option<(S, char)>,
    ) -> std::io::Result<()> {
        let url = format_link_url(url, false);
        match title {
            Some((title, quote)) if quote == ')' => {
                write!(self, r#"]({url} ({}))"#, title.as_ref())?
            }
            Some((title, quote)) => write!(self, r#"]({url} {quote}{}{quote})"#, title.as_ref())?,
            None => write!(self, "]({url})")?,
        }
        Ok(())
    }
}

pub(crate) fn format_link_url(url: &str, wrap_empty_urls: bool) -> Cow<'_, str> {
    if wrap_empty_urls && url.is_empty() {
        Cow::from("<>")
    } else if !url.starts_with('<') && !url.ends_with('>') && url.contains(' ')
        || !balanced_parens(&url)
    {
        // https://spec.commonmark.org/0.30/#link-destination
        Cow::from(format!("<{url}>"))
    } else {
        url.into()
    }
}

/// Check if the parens are balanced
fn balanced_parens(url: &str) -> bool {
    let mut stack = vec![];
    let mut was_last_escape = false;

    for b in url.bytes() {
        if !was_last_escape && b == b'(' {
            stack.push(b);
            continue;
        }

        if !was_last_escape && b == b')' {
            if let Some(top) = stack.last() {
                if *top != b'(' {
                    return false;
                }
                stack.pop();
            } else {
                return false;
            }
        }
        was_last_escape = b == b'\\';
    }
    stack.is_empty()
}

/// Search for enclosing balanced brackets
fn find_text_within_last_set_of_balance_bracket(
    label: &str,
    opener: u8,
    closer: u8,
    halt_condition: Option<fn(u8) -> bool>,
) -> (usize, usize) {
    let mut stack = vec![];
    let mut was_last_escape = false;

    let mut start = 0;
    let mut end = label.len();

    let mut bytes = label.bytes().enumerate().peekable();

    while let Some((index, byte)) = bytes.next() {
        if !was_last_escape && byte == opener {
            stack.push(index)
        }

        if !was_last_escape && byte == closer {
            if let Some(start_index) = stack.pop() {
                start = start_index;
                end = index;
            }

            if stack.is_empty() && halt_condition.is_some() {
                match (bytes.peek(), halt_condition) {
                    (Some((_, byte)), Some(halt_condition)) if halt_condition(*byte) => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        was_last_escape = byte == b'\\'
    }
    (start, end + 1)
}

/// Reference links are expected to be well formed:
/// [foo][bar] -> bar
/// [link \[bar][ref] -> ref
pub(super) fn find_reference_link_label(input: &str) -> &str {
    let (start, end) = find_text_within_last_set_of_balance_bracket(input, b'[', b']', None);
    // +1 to move past '['
    // -1 to move before ']'
    input[start + 1..end - 1].trim()
}

/// Inline links are expected to be well formed:
/// [link](/uri) -> '/uri'
/// [link](</my uri>) -> '/my uri'
pub(super) fn find_inline_url_and_title(input: &str) -> Option<(String, Option<(String, char)>)> {
    let (_, end) =
        find_text_within_last_set_of_balance_bracket(input, b'[', b']', Some(|b| b == b'('));
    // +1 to move past '('
    // -1 to move before ')'
    let inline_url = input[end + 1..input.len() - 1].trim();
    if inline_url.is_empty() {
        return Some((String::new(), None));
    }

    split_inline_url_from_title(inline_url, inline_url.ends_with(['"', '\'', ')']))
}

// The link must have a title if we're calling this
fn link_title_start(link: &[u8]) -> usize {
    let last = *link.last().expect("links titles must have quotes");
    let opener = if last == b')' { b'(' } else { last };

    // offset by 2 to skip triling quote
    let mut index = link.len() - 2;
    while index.saturating_sub(1) != 0 {
        if link[index] == opener && link[index - 1] != b'\\' {
            return index;
        }
        index -= 1;
    }

    // Odd case where a title is in the place of a url
    //https://spec.commonmark.org/0.30/#example-503
    0
}

/// Grab the link destination from the source text
///
/// `pulldown_cmark` unescape link destinations and titles so grabbing the escaped link
/// from the source is the easiest way to maintain all the escaped characters.
pub(super) fn recover_escaped_link_destination_and_title(
    complete_link: &str,
    has_title: bool,
) -> Option<(String, Option<(String, char)>)> {
    let rest = complete_link.split_once(":").map(|(_, rest)| rest.trim())?;
    split_inline_url_from_title(rest, has_title)
}

fn trim_angle_brackes(url: &str) -> &str {
    if url.starts_with('<') && url.ends_with('>') {
        url[1..url.len() - 1].trim()
    } else {
        url.trim()
    }
}

fn split_inline_url_from_title(
    input: &str,
    has_title: bool,
) -> Option<(String, Option<(String, char)>)> {
    // If both link destination and link title are present, they must be separated by spaces, tabs,
    // and up to one line ending.
    let has_space = input.contains(char::is_whitespace);
    let only_link = !has_space && has_title;
    let link_start = link_title_start(input.as_bytes());
    if only_link || !has_title || link_start == 0 {
        return Some((trim_angle_brackes(input).to_string(), None));
    }

    let (mut url, mut title_with_quotes) = input.split_at(link_start);

    url = url.trim();

    title_with_quotes = title_with_quotes.trim();

    // Remove the wrapping quotes from the title
    let quote = title_with_quotes
        .bytes()
        .last()
        .expect("url title has a quote") as char;
    let title = &title_with_quotes[1..title_with_quotes.len() - 1];

    Some((
        trim_angle_brackes(url).to_string(),
        Some((title.to_string(), quote)),
    ))
}
