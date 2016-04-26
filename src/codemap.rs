use std::cmp;
use std::iter::FromIterator;

use itertools::Itertools;

use syntax::codemap::{BytePos, CodeMap, Span};

use comment::FindUncommented;

/// A range of lines, inclusive of both ends.
#[derive(Clone, Copy, Debug, Eq, PartialEq, RustcDecodable)]
pub struct LineRange {
    pub lo: usize,
    pub hi: usize,
}

impl LineRange {
    #[inline]
    fn is_valid(self) -> bool {
        self.lo <= self.hi
    }

    #[inline]
    pub fn contains(self, other: LineRange) -> bool {
        debug_assert!(self.is_valid());
        debug_assert!(other.is_valid());

        self.lo <= other.lo && self.hi >= other.hi
    }

    #[inline]
    pub fn intersects(self, other: LineRange) -> bool {
        debug_assert!(self.is_valid());
        debug_assert!(other.is_valid());

        self.lo <= other.hi || other.lo <= self.hi
    }

    #[inline]
    /// Returns a new `LineRange` with lines from `self` and `other` if they were adjacent or
    /// intersect; returns `None` otherwise.
    pub fn merge(self, other: LineRange) -> Option<LineRange> {
        debug_assert!(self.is_valid());
        debug_assert!(other.is_valid());

        // We can't merge non-adjacent ranges.
        if self.hi + 1 < other.lo || other.hi + 1 < self.lo {
            None
        } else {
            Some(LineRange {
                lo: cmp::min(self.lo, other.lo),
                hi: cmp::max(self.hi, other.hi),
            })
        }
    }
}

/// A set of lines.
///
/// The set is represented as a list of disjoint, non-adjacent ranges sorted by lower endpoint.
/// This allows efficient querying for containment of a `LineRange`.
#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable)]
pub struct LineSet(Vec<LineRange>);

impl LineSet {
    /// Creates an empty `LineSet`.
    pub fn new() -> LineSet {
        LineSet(Vec::new())
    }

    /// Returns `true` if the lines in `range` are all contained in `self`.
    pub fn contains(&self, range: LineRange) -> bool {
        // This coule be a binary search, but it's unlikely this is a performance bottleneck.
        self.0.iter().any(|r| r.contains(range))
    }

    /// Normalizes the line ranges so they are sorted by `lo` and are disjoint: any adjacent
    /// contiguous ranges are merged.
    fn normalize(&mut self) {
        let mut v = Vec::with_capacity(self.0.len());
        {
            let ranges = &mut self.0;
            ranges.sort_by_key(|x| x.lo);
            let merged = ranges.drain(..).coalesce(|x, y| x.merge(y).ok_or((x, y)));
            v.extend(merged);
        }
        v.shrink_to_fit();

        self.0 = v;
    }
}

impl FromIterator<LineRange> for LineSet {
    /// Produce a `LineSet` from `LineRange`s in `iter`.
    fn from_iter<I: IntoIterator<Item = LineRange>>(iter: I) -> LineSet {
        let mut ret = LineSet::new();
        ret.extend(iter);

        ret
    }
}

impl Extend<LineRange> for LineSet {
    /// Add `LineRanges` from `iter` to `self`.
    fn extend<T>(&mut self, iter: T)
        where T: IntoIterator<Item = LineRange>
    {
        self.0.extend(iter);
        self.normalize();
    }
}

impl IntoIterator for LineSet {
    type Item = LineRange;
    type IntoIter = ::std::vec::IntoIter<LineRange>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub trait SpanUtils {
    fn span_after(&self, original: Span, needle: &str) -> BytePos;
    fn span_after_last(&self, original: Span, needle: &str) -> BytePos;
    fn span_before(&self, original: Span, needle: &str) -> BytePos;
}

pub trait LineRangeUtils {
    /// Returns the `LineRange` that corresponds to `span` in `self`.
    fn lookup_line_range(&self, span: Span) -> LineRange;
}

impl SpanUtils for CodeMap {
    #[inline]
    fn span_after(&self, original: Span, needle: &str) -> BytePos {
        let snippet = self.span_to_snippet(original).unwrap();
        let offset = snippet.find_uncommented(needle).unwrap() + needle.len();

        original.lo + BytePos(offset as u32)
    }

    #[inline]
    fn span_after_last(&self, original: Span, needle: &str) -> BytePos {
        let snippet = self.span_to_snippet(original).unwrap();
        let mut offset = 0;

        while let Some(additional_offset) = snippet[offset..].find_uncommented(needle) {
            offset += additional_offset + needle.len();
        }

        original.lo + BytePos(offset as u32)
    }

    #[inline]
    fn span_before(&self, original: Span, needle: &str) -> BytePos {
        let snippet = self.span_to_snippet(original).unwrap();
        let offset = snippet.find_uncommented(needle).unwrap();

        original.lo + BytePos(offset as u32)
    }
}

impl LineRangeUtils for CodeMap {
    /// Returns the `LineRange` that corresponds to `span` in `self`.
    ///
    /// # Panics
    ///
    /// Panics if `span` crosses a file boundary, which shouldn't happen.
    fn lookup_line_range(&self, span: Span) -> LineRange {
        let lo = self.lookup_char_pos(span.lo);
        let hi = self.lookup_char_pos(span.hi);

        assert!(lo.file.name == hi.file.name,
                "span crossed file boundary: lo: {:?}, hi: {:?}",
                lo,
                hi);

        LineRange {
            lo: lo.line,
            hi: hi.line,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mk_range(lo: usize, hi: usize) -> LineRange {
        assert!(lo <= hi);
        LineRange { lo: lo, hi: hi }
    }

    #[test]
    fn test_line_range_contains() {
        assert_eq!(true, mk_range(1, 2).contains(mk_range(1, 1)));
        assert_eq!(true, mk_range(1, 2).contains(mk_range(2, 2)));
        assert_eq!(false, mk_range(1, 2).contains(mk_range(0, 0)));
        assert_eq!(false, mk_range(1, 2).contains(mk_range(3, 10)));
    }

    #[test]
    fn test_line_range_merge() {
        assert_eq!(None, mk_range(1, 3).merge(mk_range(5, 5)));
        assert_eq!(None, mk_range(4, 7).merge(mk_range(0, 1)));
        assert_eq!(Some(mk_range(3, 7)), mk_range(3, 5).merge(mk_range(4, 7)));
        assert_eq!(Some(mk_range(3, 7)), mk_range(3, 5).merge(mk_range(5, 7)));
        assert_eq!(Some(mk_range(3, 7)), mk_range(3, 5).merge(mk_range(6, 7)));
        assert_eq!(Some(mk_range(3, 7)), mk_range(3, 7).merge(mk_range(4, 5)));
    }

    #[test]
    fn test_line_set_extend() {
        let mut line_set = LineSet(vec![LineRange { lo: 3, hi: 4 },
                                        LineRange { lo: 7, hi: 8 },
                                        LineRange { lo: 10, hi: 13 }]);

        // Fill in the gaps, and add one disjoint range at the end.
        line_set.extend(vec![LineRange { lo: 5, hi: 6 },
                             LineRange { lo: 9, hi: 9 },
                             LineRange { lo: 14, hi: 17 },
                             LineRange { lo: 19, hi: 21 }]);

        assert_eq!(line_set.0,
                   vec![LineRange { lo: 3, hi: 17 }, LineRange { lo: 19, hi: 21 }]);
    }
}
