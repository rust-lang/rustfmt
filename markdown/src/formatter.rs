use std::borrow::Cow;
use std::iter::Peekable;
use std::ops::Range;

use itertools::Itertools;
use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, OffsetIter, Tag};
use pulldown_cmark::{LinkDef, Options, Parser};

use crate::links;
use crate::list::{ListMarker, OrderedListMarker, UnorderedListMarker};
use crate::table::TableState;

pub struct MarkdownFormatter<'i, F> {
    /// Raw markdown input
    input: &'i str,
    pub(crate) last_was_softbreak: bool,
    /// Iterator Supplying Markdown Events
    events: Peekable<OffsetIter<'i, 'i>>,
    rewrite_buffer: String,
    /// Stores code that we might try to format
    code_block_buffer: String,
    /// Stack that keeps track of nested list markers.
    /// Unordered list markers are one of `*`, `+`, or `-`,
    /// while ordered lists markers start with 0-9 digits followed by a `.` or `)`.
    list_markers: Vec<ListMarker>,
    /// Stack that keeps track of indentation.
    indentation: Vec<Cow<'static, str>>,
    /// Stack that keeps track of whether we're formatting inside of another element.
    nested_context: Vec<Tag<'i>>,
    /// A set of reference link definitions that will be output after formatting.
    /// Reference style links contain 3 parts:
    /// 1. Text to display
    /// 2. URL
    /// 3. (Optional) Title
    /// ```markdown
    /// [title]: link "optional title"
    /// ```
    // reference_links: Vec<ReferenceLink>,
    reference_links: Vec<(String, String, Option<(String, char)>, Range<usize>)>,
    /// keep track of the current setext header.
    /// ```markdown
    /// Header
    /// ======
    /// ```
    setext_header: Option<&'i str>,
    /// next Start event should push indentation
    needs_indent: bool,
    table_state: Option<TableState<'i>>,
    last_position: usize,
    code_block_formatter: F,
}

/// Depnding on the formatting context there are a few different buffers where we might want to
/// write formatted markdown events. The StrWrite impl helps us centralize this logic.
impl<'i, F> StrWrite for MarkdownFormatter<'i, F> {
    fn write_str(&mut self, text: &str) -> std::io::Result<()> {
        if self.in_fenced_code_block() || self.in_indented_code_block() {
            self.code_block_buffer.push_str(&text);
        } else if self.in_table_header() || self.in_table_row() {
            if let Some(state) = self.table_state.as_mut() {
                state.write(text.to_owned().into());
            }
        } else {
            self.rewrite_buffer.push_str(text)
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        if self.in_fenced_code_block() || self.in_indented_code_block() {
            self.code_block_buffer.write_fmt(args)?;
        } else if self.in_table_header() || self.in_table_row() {
            if let Some(state) = self.table_state.as_mut() {
                let mut text = String::new();
                text.write_fmt(args)?;
                state.write(text.into());
            }
        } else {
            let mut text = String::new();
            text.write_fmt(args)?;
            self.rewrite_buffer.write_fmt(args)?;
        }
        Ok(())
    }
}

impl<'i, F> MarkdownFormatter<'i, F> {
    /// Peek at the next Markdown Event
    fn peek(&mut self) -> Option<&Event<'i>> {
        self.events.peek().map(|(e, _)| e)
    }

    /// Check if the next Event is an `Event::End`
    fn is_next_end_event(&mut self) -> bool {
        matches!(self.peek(), Some(Event::End(_)))
    }

    /// Check if we should write newlines and indentation before the next Start Event
    fn check_needs_indent(&mut self, event: &Event<'i>) {
        self.needs_indent = match self.peek() {
            Some(Event::Start(_) | Event::Rule | Event::Html(_) | Event::End(Tag::Item)) => true,
            Some(Event::End(Tag::BlockQuote)) => matches!(event, Event::End(_)),
            Some(Event::Text(_)) => matches!(event, Event::End(_) | Event::Start(Tag::Item)),
            _ => matches!(event, Event::Rule),
        };
    }

    /// Check if we're formatting a fenced code block
    fn in_fenced_code_block(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::CodeBlock(CodeBlockKind::Fenced(_)))
        )
    }

    /// Check if we're formatting an indented code block
    fn in_indented_code_block(&self) -> bool {
        matches!(
            self.nested_context.last(),
            Some(Tag::CodeBlock(CodeBlockKind::Indented))
        )
    }

    /// Check if we're in a paragraph
    fn in_paragraph(&self) -> bool {
        matches!(self.nested_context.last(), Some(Tag::Paragraph))
    }

    /// Check if we're in a list
    fn in_list_item(&self) -> bool {
        matches!(self.nested_context.last(), Some(Tag::Item))
    }

    // check if we're formatting a table header
    fn in_table_header(&self) -> bool {
        self.nested_context
            .iter()
            .rfind(|tag| **tag == Tag::TableHead)
            .is_some()
    }

    // check if we're formatting a table row
    fn in_table_row(&self) -> bool {
        self.nested_context
            .iter()
            .rfind(|tag| **tag == Tag::TableRow)
            .is_some()
    }

    /// Check if we're formatting in a nested context
    fn is_nested(&self) -> bool {
        !self.nested_context.is_empty()
    }

    fn count_newlines(&self, range: &Range<usize>) -> usize {
        if self.last_position == range.start {
            return 0;
        }

        let snippet = if self.last_position < range.start {
            // between two markdown evernts
            &self.input[self.last_position..range.start]
        } else {
            // likely in some nested context
            self.input[self.last_position..range.end].trim_end_matches('\n')
        };

        snippet.bytes().filter(|b| *b == b'\n').count()
    }

    fn write_indentation(&mut self, use_rewrite_buffer: bool, trim_trailing_whiltespace: bool) {
        let last_non_complete_whitespace_indent = self
            .indentation
            .iter()
            .rposition(|indent| !indent.chars().all(char::is_whitespace));

        let position = if trim_trailing_whiltespace {
            let Some(position) = last_non_complete_whitespace_indent else {
                // All indents are just whitespace. We don't want to push blank lines
                return;
            };
            position
        } else {
            self.indentation.len()
        };

        for (i, indent) in self.indentation.iter().take(position + 1).enumerate() {
            let is_last = i == position;
            let buffer = if use_rewrite_buffer {
                &mut self.rewrite_buffer
            } else {
                &mut self.rewrite_buffer
            };

            if is_last && trim_trailing_whiltespace {
                buffer.push_str(indent.trim())
            } else {
                buffer.push_str(&indent)
            }
        }
    }

    fn write_newlines(&mut self, max_newlines: usize) -> std::io::Result<()> {
        self.write_newlines_inner(max_newlines, false)
    }

    fn write_newlines_no_trailing_whitespace(
        &mut self,
        max_newlines: usize,
    ) -> std::io::Result<()> {
        self.write_newlines_inner(max_newlines, true)
    }

    fn write_newlines_inner(
        &mut self,
        max_newlines: usize,
        always_trim_trailing_whitespace: bool,
    ) -> std::io::Result<()> {
        if self.rewrite_buffer.is_empty() {
            return Ok(());
        }
        let newlines = self
            .rewrite_buffer
            .chars()
            .rev()
            .take_while(|c| *c == '\n')
            .count();

        let nested = self.is_nested();
        let newlines_to_write = max_newlines.saturating_sub(newlines);
        let next_is_end_event = self.is_next_end_event();

        for i in 0..newlines_to_write {
            let is_last = i == newlines_to_write - 1;

            self.rewrite_buffer.push('\n');

            if nested {
                self.write_indentation(true, !is_last || always_trim_trailing_whitespace);
            }
        }
        if !nested {
            self.write_indentation(true, next_is_end_event || always_trim_trailing_whitespace);
        }
        Ok(())
    }

    fn write_reference_link_inner(
        &mut self,
        label: &str,
        dest: &str,
        title: Option<&(String, char)>,
    ) -> std::io::Result<()> {
        // empty links can be specified with <>
        let dest = links::format_link_url(&dest, true);
        self.write_newlines(1)?;
        if let Some((title, quote)) = title {
            write!(self, r#"[{label}]: {dest} {quote}{title}{quote}"#)?;
        } else {
            write!(self, "[{label}]: {dest}")?;
        }
        Ok(())
    }

    fn rewrite_reference_links(&mut self, range: &Range<usize>) -> std::io::Result<()> {
        if self.reference_links.is_empty() {
            return Ok(());
        }
        // use std::mem::take to work around the borrow checker
        let mut reference_links = std::mem::take(&mut self.reference_links);

        loop {
            match reference_links.last() {
                Some((_, _, _, link_range)) if link_range.start > range.start => {
                    // The reference link on the top of the stack comes further along in the file
                    break;
                }
                None => break,
                _ => {}
            }

            let (label, dest, title, link_range) = reference_links.pop().expect("we have a value");
            let newlines = self.count_newlines(&link_range);
            self.write_newlines(newlines)?;
            self.write_reference_link_inner(&label, &dest, title.as_ref())?;
            self.last_position = link_range.end;
            self.needs_indent = true;
        }

        // put the reference_links back
        self.reference_links = reference_links;
        Ok(())
    }

    /// Write out reference links at the end of the file
    fn rewrite_final_reference_links(mut self) -> std::io::Result<String> {
        // use std::mem::take to work around the borrow checker
        let reference_links = std::mem::take(&mut self.reference_links);

        // need to iterate in reverse because reference_links is a stack
        for (label, dest, title, range) in reference_links.into_iter().rev() {
            let newlines = self.count_newlines(&range);
            self.write_newlines(newlines)?;

            // empty links can be specified with <>
            self.write_reference_link_inner(&label, &dest, title.as_ref())?;
            self.last_position = range.end
        }
        Ok(self.rewrite_buffer)
    }

    fn join_with_indentation(&mut self, buffer: &str, start_with_indentation: bool) {
        if buffer.trim().is_empty() && start_with_indentation {
            self.write_indentation(true, true);
            return;
        }

        let mut lines = buffer.trim_end().lines().peekable();
        while let Some(line) = lines.next() {
            let is_last = lines.peek().is_none();
            let is_next_empty = lines
                .peek()
                .map(|l| l.trim().is_empty())
                .unwrap_or_default();

            if start_with_indentation {
                self.write_indentation(true, line.trim().is_empty());
            }

            if !line.trim().is_empty() {
                self.rewrite_buffer.push_str(line)
            }

            if !is_last {
                self.rewrite_buffer.push('\n');
            }

            if !is_last && !start_with_indentation {
                self.write_indentation(true, is_next_empty);
            }
        }
    }
}

impl<'i, F> MarkdownFormatter<'i, F>
where
    F: Fn(&str, String) -> String,
{
    pub fn new(input: &'i str, code_block_formatter: F) -> Self {
        let mut options = Options::all();
        options.remove(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(input, options);

        let reference_links = parser
            .reference_definitions()
            .iter()
            .sorted_by(|(_, link_a), (_, link_b)| {
                // We want to sort these in descending order based on the ranges
                // This creates a stack of reference links that we can pop off of.
                link_b.span.start.cmp(&link_a.span.start)
            })
            .map(|(link_lable, LinkDef { dest, title, span })| {
                let full_link = &input[span.clone()];
                if let Some((url, title)) =
                    links::recover_escaped_link_destination_and_title(full_link, title.is_some())
                {
                    (link_lable.to_string(), url, title, span.clone())
                } else {
                    // Couldn't recover URL from source, just use what we've been given
                    (
                        link_lable.to_string(),
                        dest.to_string(),
                        title.clone().map(|s| (s.to_string(), '"')),
                        span.clone(),
                    )
                }
            })
            .collect::<Vec<_>>();

        Self {
            input,
            last_was_softbreak: false,
            events: parser.into_offset_iter().peekable(),
            rewrite_buffer: String::with_capacity(input.len() * 2),
            code_block_buffer: String::with_capacity(256),
            list_markers: vec![],
            indentation: vec![],
            nested_context: vec![],
            reference_links,
            setext_header: None,
            needs_indent: false,
            table_state: None,
            last_position: 0,
            code_block_formatter: code_block_formatter,
        }
    }

    fn format_code_buffer(&mut self, info_string: Option<&str>) -> String {
        // use std::mem::take to work around the borrow checker
        let code_block_buffer = std::mem::take(&mut self.code_block_buffer);

        let Some(info_string) = info_string else {
            // An indented code block won't have an info_string
            return code_block_buffer;
        };

        // Call the code_block_formatter fn
        (self.code_block_formatter)(info_string, code_block_buffer)
    }

    fn write_code_block_buffer(&mut self, info_string: Option<&str>) -> std::io::Result<()> {
        let code = self.format_code_buffer(info_string);
        self.join_with_indentation(&code, info_string.is_some());
        Ok(())
    }

    /// The main entry point for markdown formatting.
    pub fn format(mut self) -> std::io::Result<String> {
        while let Some((event, range)) = self.events.next() {
            let mut last_position = self.input[..range.end]
                .bytes()
                .rposition(|b| !b.is_ascii_whitespace())
                .unwrap_or(0);

            match event {
                Event::Start(tag) => {
                    self.rewrite_reference_links(&range)?;
                    last_position = range.start;
                    self.start_tag(tag.clone(), range)?;
                }
                Event::End(ref tag) => {
                    self.end_tag(tag.clone(), range)?;
                    self.check_needs_indent(&event);
                }
                Event::Text(ref parsed_text) => {
                    last_position = range.end;
                    let starts_with_escape = self.input[..range.start].ends_with('\\');
                    let newlines = self.count_newlines(&range);
                    let text_from_source = CowStr::from(&self.input[range]);
                    let text = if text_from_source.is_empty() {
                        // This seems to happen when the parsed text is whitespace only.
                        // To preserve leading whitespace use the parsed text instead.
                        parsed_text
                    } else {
                        &text_from_source
                    };
                    if self.needs_indent
                        && newlines > 0
                        && (self.in_paragraph() || self.in_list_item())
                    {
                        self.write_newlines(newlines)?;
                        self.needs_indent = false;
                    }

                    if starts_with_escape || self.needs_escape(&text) {
                        // recover escape characters
                        write!(self, "\\{text}")?;
                    } else {
                        write!(self, "{text}")?;
                    }
                    self.check_needs_indent(&event);
                }
                Event::Code(_) => {
                    write!(self, "{}", &self.input[range])?;
                }
                Event::SoftBreak => {
                    last_position = range.end;
                    write!(self, "{}", &self.input[range])?;
                    self.write_indentation(true, false);
                    self.last_was_softbreak = true;
                }
                Event::HardBreak => {
                    write!(self, "{}", &self.input[range])?;
                }
                Event::Html(_) => {
                    let newlines = self.count_newlines(&range);
                    if self.needs_indent {
                        self.write_newlines(newlines)?;
                    }
                    write!(self, "{}", &self.input[range].trim_end_matches('\n'))?;
                    self.check_needs_indent(&event);
                }
                Event::Rule => {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    write!(self, "{}", &self.input[range])?;
                    self.check_needs_indent(&event)
                }
                Event::FootnoteReference(text) => {
                    write!(self, "[^{text}]")?;
                }
                Event::TaskListMarker(done) => {
                    if done {
                        write!(self, "[x]")?;
                    } else {
                        write!(self, "[ ]")?;
                    }
                }
            }
            self.last_position = last_position
        }
        debug_assert!(self.nested_context.is_empty());
        self.rewrite_final_reference_links()
    }

    fn start_tag(&mut self, tag: Tag<'i>, range: Range<usize>) -> std::io::Result<()> {
        use Tag::*;
        match tag {
            Paragraph => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                self.nested_context.push(tag);
            }
            Heading(level, _, _) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                let full_header = self.input[range].trim();

                if full_header.contains('\n') && full_header.ends_with(['=', '-']) {
                    // support for alternative syntax for H1 and H2
                    // <https://www.markdownguide.org/basic-syntax/#alternate-syntax>
                    let header_marker = full_header.split('\n').last().unwrap().trim();
                    self.setext_header.replace(header_marker);
                    // setext header are handled in `end_tag`
                    return Ok(());
                }

                let header = match level {
                    HeadingLevel::H1 => "# ",
                    HeadingLevel::H2 => "## ",
                    HeadingLevel::H3 => "### ",
                    HeadingLevel::H4 => "#### ",
                    HeadingLevel::H5 => "##### ",
                    HeadingLevel::H6 => "###### ",
                };

                let empty_header = full_header
                    .trim_start_matches(header)
                    .trim_end_matches(|c: char| c.is_whitespace() || matches!(c, '#' | '\\'))
                    .is_empty();

                if empty_header {
                    write!(self, "{}", header.trim())?;
                } else {
                    write!(self, "{header}")?;
                }
            }
            BlockQuote => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                if matches!(self.peek(), Some(Event::End(BlockQuote))) {
                    // Special case handling for empty block quotes
                    let block_quote_opener =
                        self.input[range].bytes().filter(|b| *b == b'>').count();
                    for i in 0..block_quote_opener {
                        let is_last = i == block_quote_opener - 1;
                        write!(self, ">")?;
                        if !is_last {
                            write!(self, "\n")?;
                        }
                        self.write_indentation(true, false);
                    }
                } else {
                    self.write_str("> ")?;
                }

                self.nested_context.push(tag);
                self.indentation.push("> ".into());
            }
            CodeBlock(ref kind) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                match kind {
                    CodeBlockKind::Fenced(info_string) => {
                        rewrite_marker(self.input, &range, &mut self.rewrite_buffer)?;

                        if info_string.is_empty() {
                            write!(self, "\n")?;
                            self.nested_context.push(tag);
                            return Ok(());
                        }

                        let starts_with_space = self.input[range.clone()]
                            .trim_start_matches(['`', '~'])
                            .starts_with(char::is_whitespace);

                        let info_string = self.input[range]
                            .lines()
                            .next()
                            .unwrap_or_else(|| info_string.as_ref())
                            .trim_start_matches(['`', '~'])
                            .trim();

                        if starts_with_space {
                            write!(self, " {info_string}\n")?;
                        } else {
                            write!(self, "{info_string}\n")?;
                        }
                    }
                    CodeBlockKind::Indented => {
                        // TODO(ytmimi) support tab as an indent
                        let indentation = "    ";

                        if !matches!(
                            self.peek(),
                            Some(Event::End(CodeBlock(CodeBlockKind::Indented)))
                        ) {
                            // Only write indentation if this isn't an empty indented code block
                            self.write_str(indentation)?;
                        }

                        self.indentation.push(indentation.into());
                    }
                }
                self.nested_context.push(tag);
            }
            List(value) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                let list_marker = if let Some(number) = value {
                    let marker = match self.input[range.clone()]
                        .chars()
                        .find(|c| ['.', ')'].contains(c))
                        .expect("we should find ordered list markers")
                    {
                        '.' => OrderedListMarker::Period,
                        ')' => OrderedListMarker::Parenthesis,
                        _ => unreachable!(),
                    };
                    let zero_padding = if number != 0 {
                        self.input[range]
                            .trim_start()
                            .bytes()
                            .take_while(|b| *b == b'0')
                            .count()
                    } else {
                        0
                    };

                    ListMarker::Ordered {
                        zero_padding: zero_padding,
                        number: number as usize,
                        marker,
                    }
                } else {
                    let marker = match self.input[range]
                        .chars()
                        .find(|c| ['*', '+', '-'].contains(c))
                        .expect("we should find unorder list marker")
                    {
                        '*' => UnorderedListMarker::Asterisk,
                        '+' => UnorderedListMarker::Plus,
                        '-' => UnorderedListMarker::Hyphen,
                        _ => unreachable!(),
                    };
                    ListMarker::Unordered(marker)
                };

                self.list_markers.push(list_marker);
                self.nested_context.push(tag);
            }
            Item => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                let empty_list_item = matches!(self.peek(), Some(Event::End(Item)));

                // Take list_marker so we can use `write!(self, ...)`
                let mut list_marker = self
                    .list_markers
                    .pop()
                    .expect("can't have list item without marker");
                let marker_char = list_marker.marker_char();
                match &list_marker {
                    ListMarker::Ordered { number, .. } if empty_list_item => {
                        let zero_padding = list_marker.zero_padding();
                        write!(self, "{zero_padding}{number}{marker_char}")?;
                    }
                    ListMarker::Ordered { number, .. } => {
                        let zero_padding = list_marker.zero_padding();
                        write!(self, "{zero_padding}{number}{marker_char} ")?;
                    }
                    ListMarker::Unordered(_) if empty_list_item => {
                        write!(self, "{marker_char}")?;
                    }
                    ListMarker::Unordered(_) => {
                        write!(self, "{marker_char} ")?;
                    }
                }

                self.nested_context.push(tag);
                // Increment the list marker in case this is a ordered list and
                // swap the list marker we took earlier
                list_marker.increment_count();
                self.indentation.push(list_marker.indentation());
                self.list_markers.push(list_marker)
            }
            FootnoteDefinition(label) => {
                write!(self, "[^{label}]: ")?;
            }
            Emphasis => {
                rewrite_marker_with_limit(self.input, &range, self, Some(1))?;
            }
            Strong => {
                rewrite_marker_with_limit(self.input, &range, self, Some(2))?;
            }
            Strikethrough => {
                rewrite_marker(self.input, &range, self)?;
            }
            Link(link_type, ..) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                let email_or_auto = matches!(link_type, LinkType::Email | LinkType::Autolink);
                let opener = if email_or_auto { "<" } else { "[" };
                self.write_str(opener)?;
                self.nested_context.push(tag);
            }
            Image(..) => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }

                write!(self, "![")?;
                self.nested_context.push(tag);
            }
            Table(ref alignment) => {
                if self.needs_indent {
                    let newlines = self.count_newlines(&range);
                    self.write_newlines(newlines)?;
                    self.needs_indent = false;
                }
                self.table_state.replace(TableState::new(alignment.clone()));
                write!(self, "|")?;
                self.indentation.push("|".into());
                self.nested_context.push(tag);
            }
            TableHead => {
                self.nested_context.push(tag);
            }
            TableRow => {
                self.nested_context.push(tag);
                if let Some(state) = self.table_state.as_mut() {
                    state.push_row()
                }
            }
            TableCell => {
                if !matches!(self.peek(), Some(Event::End(TableCell))) {
                    return Ok(());
                }

                if let Some(state) = self.table_state.as_mut() {
                    state.write("".into());
                }
            }
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: Tag<'i>, range: Range<usize>) -> std::io::Result<()> {
        use Tag::*;
        match tag {
            Paragraph => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
            }
            Heading(_, fragment_identifier, classes) => {
                match (fragment_identifier, classes.is_empty()) {
                    (Some(id), false) => {
                        let classes = rewirte_header_classes(classes)?;
                        write!(self, " {{#{id}{classes}}}")?;
                    }
                    (Some(id), true) => {
                        write!(self, " {{#{id}}}")?;
                    }
                    (None, false) => {
                        let classes = rewirte_header_classes(classes)?;
                        write!(self, " {{{}}}", classes.trim())?;
                    }
                    (None, true) => {}
                }

                if let Some(marker) = self.setext_header.take() {
                    self.write_newlines(1)?;
                    write!(self, "{marker}")?;
                }
            }
            BlockQuote => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    // Recover empty block quote lines
                    if let Some(last) = self.indentation.last_mut() {
                        // Avoid trailing whitespace by replacing the last indentation with '>'
                        *last = ">".into()
                    }
                    self.write_newlines(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));

                let popped_indentation = self
                    .indentation
                    .pop()
                    .expect("we pushed a blockquote marker in start_tag");
                if let Some(indentation) = self.indentation.last_mut() {
                    if indentation == ">" {
                        *indentation = popped_indentation
                    }
                }
            }
            CodeBlock(ref kind) => {
                match kind {
                    CodeBlockKind::Fenced(info_string) => {
                        self.write_code_block_buffer(Some(info_string))?;
                        // write closing code fence
                        self.write_newlines(1)?;
                        rewrite_marker(self.input, &range, &mut self.rewrite_buffer)?;
                    }
                    CodeBlockKind::Indented => {
                        // Maybe we'll consider formatting indented code blocks??
                        self.write_code_block_buffer(None)?;

                        let popped_indentation = self
                            .indentation
                            .pop()
                            .expect("we added 4 spaces in start_tag");
                        debug_assert_eq!(popped_indentation, "    ");
                    }
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
            }
            List(_) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                self.list_markers.pop();

                // To prevent the next code block from being interpreted as a list we'll add an
                // HTML comment See https://spec.commonmark.org/0.30/#example-308, which states:
                //
                //     To separate consecutive lists of the same type, or to separate a list from an
                //     indented code block that would otherwise be parsed as a subparagraph of the
                //     final list item, you can insert a blank HTML comment
                if let Some(Event::Start(Tag::CodeBlock(CodeBlockKind::Indented))) = self.peek() {
                    self.write_newlines(1)?;
                    write!(self, "<!-- Dont absorb code block into list -->\n")?;
                    write!(self, "<!-- Consider a feenced code block instead -->")?;
                };
            }
            Item => {
                let newlines = self.count_newlines(&range);
                if self.needs_indent && newlines > 0 {
                    self.write_newlines_no_trailing_whitespace(newlines)?;
                }
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                let popped_indentation = self
                    .indentation
                    .pop()
                    .expect("we list item indentation in start_tag");

                // There should always be a list marker since we don't pop
                // list markers off the stack until we've reached the end of the list
                if let Some(marker) = self.list_markers.last() {
                    debug_assert_eq!(marker.indentation(), popped_indentation);
                }

                // if the next event is a Start(Item), then we need to set needs_indent
                self.needs_indent = matches!(self.peek(), Some(Event::Start(Item)));
            }
            FootnoteDefinition(_label) => {}
            Emphasis => {
                rewrite_marker_with_limit(self.input, &range, self, Some(1))?;
            }
            Strong => {
                rewrite_marker_with_limit(self.input, &range, self, Some(2))?;
            }
            Strikethrough => {
                rewrite_marker(self.input, &range, self)?;
            }
            Link(ref link_type, ref url, ref title) | Image(ref link_type, ref url, ref title) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag.as_ref(), Some(&tag));

                let text = &self.input[range.clone()];

                match link_type {
                    LinkType::Inline => {
                        if let Some((source_url, title_and_quote)) =
                            crate::links::find_inline_url_and_title(text)
                        {
                            self.write_inline_link(&source_url, title_and_quote)?;
                        } else {
                            let title = if title.is_empty() {
                                None
                            } else {
                                Some((title, '"'))
                            };
                            self.write_inline_link(&url, title)?;
                        }
                    }
                    LinkType::Reference | LinkType::ReferenceUnknown => {
                        let label = crate::links::find_reference_link_label(text);
                        write!(self, "][{label}]")?;
                    }
                    LinkType::Collapsed | LinkType::CollapsedUnknown => write!(self, "][]")?,
                    LinkType::Shortcut | LinkType::ShortcutUnknown => write!(self, "]")?,
                    LinkType::Autolink | LinkType::Email => write!(self, ">")?,
                }
            }
            Table(_) => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
                if let Some(state) = self.table_state.take() {
                    self.join_with_indentation(&state.format()?, false)
                }
                let popped_indentation = self.indentation.pop().expect("we added `|` in start_tag");
                debug_assert_eq!(popped_indentation, "|");
            }
            TableRow | TableHead => {
                let popped_tag = self.nested_context.pop();
                debug_assert_eq!(popped_tag, Some(tag));
            }
            TableCell => {
                if let Some(state) = self.table_state.as_mut() {
                    // We finished formatting this cell. Setup the state to format the next cell
                    state.increment_col_index()
                }
            }
        }
        Ok(())
    }
}

/// Find some marker that denotes the start of a markdown construct.
/// for example, `**` for bold or `_` for italics.
fn find_marker<'i, P>(input: &'i str, range: &Range<usize>, predicate: P) -> &'i str
where
    P: FnMut(char) -> bool,
{
    let end = if let Some(position) = input[range.start..].chars().position(predicate) {
        range.start + position
    } else {
        range.end
    };
    &input[range.start..end]
}

/// Find some marker, but limit the size
fn rewrite_marker_with_limit<'i, W: StrWrite>(
    input: &'i str,
    range: &Range<usize>,
    writer: &mut W,
    size_limit: Option<usize>,
) -> std::io::Result<()> {
    let marker_char = input[range.start..].chars().next().unwrap();
    let marker = find_marker(input, &range, |c| c != marker_char);
    if let Some(mark_max_width) = size_limit {
        write!(writer, "{}", &marker[..mark_max_width])
    } else {
        write!(writer, "{marker}")
    }
}

/// Finds a marker in the source text and writes it to the buffer
fn rewrite_marker<'i, W: StrWrite>(
    input: &'i str,
    range: &Range<usize>,
    writer: &mut W,
) -> std::io::Result<()> {
    rewrite_marker_with_limit(input, range, writer, None)
}

/// Rewrite a list of h1, h2, h3, h4, h5, h6 classes
fn rewirte_header_classes(classes: Vec<&str>) -> std::io::Result<String> {
    let item_len = classes.iter().map(|i| i.len()).sum::<usize>();
    let capacity = item_len + classes.len() * 2;
    let mut result = String::with_capacity(capacity);
    for class in classes {
        write!(result, " .{class}")?;
    }
    Ok(result)
}
