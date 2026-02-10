use std::{
    ops::{Bound, Deref, RangeBounds},
    str::{CharIndices, Chars},
};

use nom::{Compare, CompareResult, Input as NomInput, Needed};
use swc_common::{BytePos, Span, comments::Comment};

use crate::ast::Text;

#[derive(Debug, Clone, Copy)]
pub struct Input<'i> {
    start: BytePos,
    end: BytePos,
    src: &'i str,
}

impl<'a> From<&'a Comment> for Input<'a> {
    fn from(c: &'a Comment) -> Self {
        Self::new(c.span.lo, c.span.hi, &c.text)
    }
}

impl<'i> Input<'i> {
    pub const fn empty() -> Self {
        Self::new(BytePos::DUMMY, BytePos::DUMMY, "")
    }

    pub const fn new(start: BytePos, end: BytePos, src: &'i str) -> Self {
        Self { start, end, src }
    }

    #[inline(always)]
    pub fn span(self) -> Span {
        Span::new(self.start, self.end)
    }

    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => self.src.len(),
        };

        let s = &self.src[start..end];
        Self::new(
            self.start + BytePos(start as _),
            self.start + BytePos(end as _),
            s,
        )
    }

    pub fn iter_indices(&self) -> CharIndices<'i> {
        self.src.char_indices()
    }
}

impl From<Input<'_>> for Text {
    fn from(i: Input) -> Self {
        Self {
            span: Span::new(i.start, i.end),
            value: i.src.into(),
        }
    }
}

impl<'a> Compare<&'a str> for Input<'_> {
    fn compare(&self, t: &'a str) -> CompareResult {
        self.src.compare(t)
    }

    fn compare_no_case(&self, t: &'a str) -> CompareResult {
        self.src.compare_no_case(t)
    }
}

impl<'a> NomInput for Input<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn input_len(&self) -> usize {
        self.src.len()
    }

    fn take(&self, index: usize) -> Self {
        let s = &self.src[..index];
        Self::new(self.start, self.start + BytePos(index as _), s)
    }

    fn take_from(&self, index: usize) -> Self {
        let s = &self.src[index..];
        Self::new(self.start + BytePos(index as _), self.end, s)
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.src.split_at(index);
        let mid = self.start + BytePos(index as _);
        (
            Self::new(mid, self.end, suffix),
            Self::new(self.start, mid, prefix),
        )
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.src.find(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.src.chars()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.src.char_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        let mut cnt = 0;
        for (index, _) in self.src.char_indices() {
            if cnt == count {
                return Ok(index);
            }
            cnt += 1;
        }

        if cnt == count {
            return Ok(self.src.len());
        }

        Err(Needed::Unknown)
    }
}

impl Deref for Input<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.src
    }
}
