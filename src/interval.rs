use std::{cmp::Ordering, ops::Bound};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Interval<I> {
    start: Bound<I>,
    end: Bound<I>,
}

impl<I> Interval<I> {
    pub fn new(s: I, e: I) -> Self {
        Self {
            start: Bound::Included(s),
            end: Bound::Included(e),
        }
    }
}

impl<I> PartialOrd for Interval<I>
where
    I: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.start < other.start && self.end < other.end {
            Some(Ordering::Less)
        } else if self.start > other.end && self.end > other.end {
            Some(Ordering::Greater)
        } else if self.start == other.start && self.end == other.end {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}
