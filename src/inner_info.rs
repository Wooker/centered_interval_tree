use std::fmt::{Debug, Display};

use crate::interval::Interval;

#[derive(PartialEq, Debug, Clone)]
pub struct InnerInfo<I, V> {
    value: V,
    interval: Interval<I>,
}

impl<I, V> InnerInfo<I, V> {
    pub fn interval(&self) -> &Interval<I> {
        &self.interval
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<I, V> Display for InnerInfo<I, V>
where
    I: Display + Debug,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Value: {}\nInterval: {:?}\n",
            self.value(),
            self.interval()
        ))
    }
}
