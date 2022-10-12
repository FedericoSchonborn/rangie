#![warn(clippy::pedantic)]
#![feature(step_trait)]

use std::{iter::Step, ops::Range as StdRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range<Idx> {
    start: Idx,
    end: Idx,
}

impl<Idx> Range<Idx> {
    pub fn new(start: Idx, end: Idx) -> Self {
        Self { start, end }
    }

    pub fn into_std(self) -> StdRange<Idx> {
        self.start..self.end
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

impl<Idx> IntoIterator for Range<Idx>
where
    Idx: Step,
{
    type Item = Idx;
    type IntoIter = StdRange<Idx>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_std()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range as StdRange;

    use crate::Range;

    #[test]
    fn from() {
        assert_eq!(Range::from(0..3).into_std(), StdRange { start: 0, end: 3 });
    }
}
