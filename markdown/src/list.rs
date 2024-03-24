use std::borrow::Cow;
// Including all these spaces might be overkill, but it probably doesn't hurt.
// In practice we'll see far fewer digits in an ordered list.
//
// <https://github.github.com/gfm/#list-items> mentions that:
//
//     An ordered list marker is a sequence of 1–9 arabic digits (0-9), followed by either a .
//     character or a ) character. (The reason for the length limit is that with 10 digits we
//     start seeing integer overflows in some browsers.)
//
const LIST_INDENTATION: &'static str = "                    ";
const ZERO_PADDING: &'static str = "00000000000000000000";

#[derive(Debug)]
pub(super) enum ListMarker {
    Ordered {
        zero_padding: usize,
        number: usize,
        marker: OrderedListMarker,
    },
    Unordered(UnorderedListMarker),
}

impl std::default::Default for ListMarker {
    fn default() -> Self {
        ListMarker::Unordered(UnorderedListMarker::Asterisk)
    }
}

impl ListMarker {
    pub(super) fn increment_count(&mut self) {
        match self {
            Self::Ordered { number, .. } => {
                *number += 1;
            }
            Self::Unordered(_) => {}
        }
    }

    pub(super) fn indentation(&self) -> Cow<'static, str> {
        let indent_index = self.indentation_len();

        if indent_index <= LIST_INDENTATION.len() {
            Cow::from(&LIST_INDENTATION[..indent_index])
        } else {
            // I think it would be extreamly rare to hit his case
            Cow::from(" ".repeat(indent_index))
        }
    }

    pub(super) fn marker_char(&self) -> char {
        match self {
            Self::Ordered { marker, .. } => marker.into(),
            Self::Unordered(marker) => marker.into(),
        }
    }

    pub(super) fn zero_padding(&self) -> &'static str {
        match self {
            Self::Ordered { zero_padding, .. } => &ZERO_PADDING[..zero_padding.clone()],
            Self::Unordered(_) => "",
        }
    }

    fn indentation_len(&self) -> usize {
        match self {
            Self::Ordered {
                zero_padding,
                number,
                ..
            } => {
                let char_len = number.checked_ilog10().unwrap_or(0) + 1;
                // + 2 to for '. '
                zero_padding + (char_len + 2) as usize
            }
            Self::Unordered(_) => 2,
        }
    }
}

#[derive(Debug)]
pub(super) enum OrderedListMarker {
    Period,
    Parenthesis,
}

impl From<OrderedListMarker> for char {
    fn from(value: OrderedListMarker) -> Self {
        match value {
            OrderedListMarker::Period => '.',
            OrderedListMarker::Parenthesis => ')',
        }
    }
}

impl From<&OrderedListMarker> for char {
    fn from(value: &OrderedListMarker) -> Self {
        match value {
            OrderedListMarker::Period => '.',
            OrderedListMarker::Parenthesis => ')',
        }
    }
}

#[derive(Debug)]
pub(super) enum UnorderedListMarker {
    Asterisk,
    Plus,
    Hyphen,
}

impl From<UnorderedListMarker> for char {
    fn from(value: UnorderedListMarker) -> Self {
        match value {
            UnorderedListMarker::Asterisk => '*',
            UnorderedListMarker::Plus => '+',
            UnorderedListMarker::Hyphen => '-',
        }
    }
}
impl From<&UnorderedListMarker> for char {
    fn from(value: &UnorderedListMarker) -> Self {
        match value {
            UnorderedListMarker::Asterisk => '*',
            UnorderedListMarker::Plus => '+',
            UnorderedListMarker::Hyphen => '-',
        }
    }
}
