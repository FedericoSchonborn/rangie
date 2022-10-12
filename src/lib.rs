#![warn(clippy::pedantic)]
#![feature(step_trait)]

//! An alternative to [`std::ops::Range`] that implements [`IntoIterator`] instead of being an
//! [`Iterator`] itself, allowing it to also implement [`Copy`].

use std::{iter::Step, ops::Range as StdRange};

/// An alternative to [`std::ops::Range`] that implements [`IntoIterator`] instead of being an
/// [`Iterator`] itself, allowing it to also implement [`Copy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range<Idx> {
    start: Idx,
    end: Idx,
}

impl<Idx> Range<Idx> {
    /// Create a new [`Range`].
    pub fn new(start: Idx, end: Idx) -> Self {
        Self { start, end }
    }
}

impl<Idx> From<StdRange<Idx>> for Range<Idx> {
    fn from(it: StdRange<Idx>) -> Self {
        Self {
            start: it.start,
            end: it.end,
        }
    }
}

impl<Idx> From<Range<Idx>> for StdRange<Idx> {
    fn from(it: Range<Idx>) -> Self {
        Self {
            start: it.start,
            end: it.end,
        }
    }
}

impl<Idx> IntoIterator for Range<Idx>
where
    Idx: Step,
{
    type Item = Idx;
    /// Why make new type when `std` do trick?
    type IntoIter = StdRange<Idx>;

    fn into_iter(self) -> Self::IntoIter {
        StdRange {
            start: self.start,
            end: self.end,
        }
    }
}
