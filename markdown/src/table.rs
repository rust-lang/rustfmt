use crate::utils::unicode_str_width;
use itertools::{EitherOrBoth, Itertools};
use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::Alignment;
use unicode_segmentation::UnicodeSegmentation;
use std::borrow::Cow;

pub(super) struct TableState<'a> {
    /// Alignment markers for HTML rendering
    /// * :-: center alignment
    /// * :-- left alignment
    /// * --: right alignmet
    /// * --- no alignmet
    alignment: Vec<Alignment>,
    /// Table headers
    headers: Vec<Cow<'a, str>>,
    /// Keep track of the widest cell in each column.
    /// Will use this info to align columns when rewriting
    max_column_width: Vec<usize>,
    /// Table rows
    body: Vec<Vec<Cow<'a, str>>>,
    /// Keep track of whether or not we're writing to the headers
    write_to_body: bool,
    /// Keep track of Which cell we're currently operating on.
    col_index: usize,
}

impl<'a> TableState<'a> {
    pub(super) fn new(alignment: Vec<Alignment>) -> Self {
        let capacity = alignment.len();
        Self {
            alignment,
            headers: Vec::with_capacity(capacity),
            max_column_width: vec![3; capacity],
            body: vec![],
            write_to_body: false,
            col_index: 0,
        }
    }

    /// Write some values to the table state.
    pub(super) fn write(&mut self, value: Cow<'a, str>) {
        if self.write_to_body {
            // push or update the body
            self.write_cell(value)
        } else {
            // push or update the headers
            self.write_header(value)
        }
    }

    /// Allows the caller to advance the internal col_index.
    /// All subsequent calls to `write` will affect the next cell
    pub(super) fn increment_col_index(&mut self) {
        self.col_index += 1;
    }

    /// Update the table state to start writing to the table body
    pub(super) fn push_row(&mut self) {
        self.body.push(Vec::with_capacity(self.alignment.len()));
        self.write_to_body = true;
        self.col_index = 0;
    }

    fn write_header(&mut self, text: Cow<'a, str>) {
        let header_width = unicode_str_width(&text);
        if let Some(column_header) = self.headers.get_mut(self.col_index) {
            *column_header += text;

            let current_width = self
                .max_column_width
                .get_mut(self.col_index)
                .expect("should be set");
            *current_width += header_width;
        } else {
            self.headers.push(text);
            self.update_column_width(self.col_index, header_width);
        }
    }

    fn write_cell(&mut self, text: Cow<'a, str>) {
        let row = self
            .body
            .last_mut()
            .expect("can only write cells after push_row called");

        if let Some(cell_value) = row.get_mut(self.col_index) {
            let current_width = unicode_str_width(cell_value) + unicode_str_width(&text);
            *cell_value += text;
            self.update_column_width(self.col_index, current_width);
        } else {
            let cell_width = unicode_str_width(&text);
            row.push(text);
            self.update_column_width(self.col_index, cell_width);
        }
    }

    fn update_column_width(&mut self, index: usize, column_width: usize) {
        if let Some(old_column_width) = self.max_column_width.get_mut(index) {
            if *old_column_width < column_width {
                *old_column_width = column_width
            }
        }
    }

    pub(super) fn format(self) -> std::io::Result<String> {
        let mut result = String::new();
        self.rewrite_header(&mut result)?;
        self.rewrite_alignment(&mut result)?;
        self.rewrite_body(&mut result)?;
        Ok(result)
    }

    fn write_wth_padding(buffer: &mut String, value: &str, mut size: usize) -> std::io::Result<()> {
        let offset = UnicodeSegmentation::graphemes(value, true).map(|grapheme| unicode_str_width(grapheme).saturating_sub(1)).sum();
        size = size.saturating_sub(offset);
        write!(buffer, " {value:<0$} |", size)
    }

    fn rewrite_header(&self, buffer: &mut String) -> std::io::Result<()> {
        for (header, width) in self.headers.iter().zip(self.max_column_width.iter()) {
            Self::write_wth_padding(buffer, header, *width)?;
        }
        Ok(())
    }

    fn rewrite_alignment(&self, buffer: &mut String) -> std::io::Result<()> {
        buffer.push('\n');
        for (alignment, width) in self.alignment.iter().zip(self.max_column_width.iter()) {
            let alignment = match alignment {
                Alignment::Center => {
                    // :-:
                    // (- 2 to account for 2 `:`)
                    format!(":{:-^1$}:", "-", width - 2)
                }
                Alignment::Left => {
                    // :--
                    format!("{:-<1$}", ":", width)
                }
                Alignment::Right => {
                    // --:
                    format!("{:->1$}", ":", width)
                }
                Alignment::None => {
                    // ---
                    "-".repeat(*width)
                }
            };
            Self::write_wth_padding(buffer, &alignment, *width)?;
        }
        Ok(())
    }

    fn rewrite_body(&self, buffer: &mut String) -> std::io::Result<()> {
        for row in self.body.iter() {
            buffer.push('\n');
            for either_or_both in row.iter().zip_longest(self.max_column_width.iter()) {
                match either_or_both {
                    EitherOrBoth::Both(cell, width) => {
                        Self::write_wth_padding(buffer, cell, *width)?;
                    }
                    EitherOrBoth::Right(width) => {
                        Self::write_wth_padding(buffer, "", *width)?;
                    }
                    EitherOrBoth::Left(_) => {
                        // There may be fewer cells in a row, but there should never be more cells.
                        // See https://github.github.com/gfm/#example-204 and the preceding text
                        unreachable!()
                    }
                }
            }
        }
        Ok(())
    }
}
