use std::{fmt::Display, ops::Add};

#[derive(Debug)]
pub enum OverlapOrdering {
    SuperSet,            // [1, 4] in relation to [2, 3]
    SubSet,              // [2, 3] in relation to [1, 4]
    Less,                // [1, 2] in relation to [3, 4]
    OverlapLess,         // [1, 3] in relation to [2, 4]
    OverlapEqualLess,    // [1, 2] in relation to [2, 4]
    Greater,             // [3, 4] in relation to [1, 2]
    OverlapGreater,      // [2, 4] in relation to [1, 3]
    OverlapEqualGreater, // [3, 4] in relation to [1, 3]
    Equal,               // [1, 2] [1, 2]
    NotPossible,
}

#[allow(unused)]
macro_rules! interval {
    // Match an interval of the form [start, end]
    ([$start:expr, $end:expr]) => {{
        Interval::new($start, $end)
    }};
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Interval<I>
where
    I: std::fmt::Debug,
{
    start: I,
    end: I,
}

impl<I> Display for Interval<I>
where
    I: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:?}, {:?}]", self.start, self.end))
    }
}

impl<I> Interval<I>
where
    I: PartialEq + PartialOrd + std::fmt::Debug,
{
    pub fn new(s: I, e: I) -> Self {
        Self { start: s, end: e }
    }

    pub fn start(&self) -> &I {
        &self.start
    }

    pub fn end(&self) -> &I {
        &self.end
    }

    pub fn compared_to(&self, other: &Self) -> OverlapOrdering {
        let ordering = match (
            self.start.partial_cmp(&other.end),
            self.end.partial_cmp(&other.start),
        ) {
            (None, None) => OverlapOrdering::NotPossible,
            (None, Some(_)) => OverlapOrdering::NotPossible,
            (Some(_), None) => OverlapOrdering::NotPossible,
            (Some(se), Some(es)) => {
                match (se, es) {
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                        OverlapOrdering::Greater
                    }
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                        OverlapOrdering::OverlapEqualLess
                    }
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                        OverlapOrdering::Equal
                    }
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                        OverlapOrdering::OverlapEqualGreater
                    }
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                        OverlapOrdering::Less
                    }
                    // SuperSet or SubSet
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                        match (
                            self.start.partial_cmp(&other.start),
                            self.end.partial_cmp(&other.end),
                        ) {
                            (None, None) => OverlapOrdering::NotPossible,
                            (None, Some(_)) => OverlapOrdering::NotPossible,
                            (Some(_), None) => OverlapOrdering::NotPossible,
                            (Some(ss), Some(ee)) => match (ss, ee) {
                                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                                    OverlapOrdering::SuperSet
                                }
                                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                                    OverlapOrdering::OverlapGreater
                                }
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                                    OverlapOrdering::SuperSet
                                }
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                                    OverlapOrdering::SuperSet
                                }
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                                    OverlapOrdering::Equal
                                }
                                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                                    OverlapOrdering::SuperSet
                                }
                                (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                                    OverlapOrdering::SubSet
                                }
                                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                                    OverlapOrdering::OverlapLess
                                }
                                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                                    OverlapOrdering::OverlapGreater
                                }
                            },
                        }
                    }
                    _ => {
                        println!("Not possible");
                        OverlapOrdering::NotPossible
                    }
                }
            }
        };

        ordering
    }

    pub fn compare_point(&self, other: &I) -> OverlapOrdering {
        match self.start.partial_cmp(&other) {
            Some(_) => todo!(),
            None => OverlapOrdering::NotPossible,
        }
    }
}

impl<I> Add<Interval<I>> for Interval<I>
where
    I: std::fmt::Debug + PartialOrd,
{
    type Output = Self;

    fn add(self, rhs: Interval<I>) -> Self::Output {
        println!("{}", self);
        println!("{}", rhs);

        let start = (self.start > rhs.start)
            .then(|| rhs.start)
            .or(Some(self.start))
            .unwrap();
        let end = (self.end < rhs.end)
            .then(|| rhs.end)
            .or(Some(self.end))
            .unwrap();

        let int = interval!([start, end]);
        dbg!(&int);
        int
    }
}
