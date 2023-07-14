#[derive(Debug, PartialEq, Clone)]
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
