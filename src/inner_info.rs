use std::fmt::{Debug, Display};

use crate::interval::Interval;

#[derive(PartialEq, Debug, Clone)]
pub struct InnerInfo<I: std::fmt::Debug, V> {
    pub(crate) value: V,
    pub(crate) interval: Interval<I>,
    pub(crate) full_interval: Interval<I>,
}

impl<I, V> InnerInfo<I, V>
where
    I: std::fmt::Debug,
{
    pub fn interval(&self) -> &Interval<I> {
        &self.interval
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn full_interval(&self) -> &Interval<I> {
        &self.full_interval
    }
}

impl<I, V> Display for InnerInfo<I, V>
where
    I: Display + Debug,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}, {}, {}\n",
            self.value(),
            self.interval,
            self.full_interval()
        ))
    }
}
