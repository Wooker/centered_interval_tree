pub enum OverlapOrdering {
    SuperSet,            // (1, 4) in relation to (2, 3)
    SubSet,              // (2, 3) in relation to (1, 4)
    Less,                // (1, 2) in relation to (3, 4)
    OverlapLess,         // (1, 3) in relation to (2, 4)
    OverlapEqualLess,    // (1, 2) in relation to (2, 4)
    Greater,             // (3, 4) in relation to (1, 2)
    OverlapGreater,      // (2, 4) in relation to (1, 3)
    OverlapEqualGreater, // (3, 4) in relation to (1, 3)
    Equal,               // (1, 2) (1, 2)
    NoOverlap,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Interval<I> {
    start: I,
    end: I,
}

impl<I> Interval<I>
where
    I: PartialEq + PartialOrd,
{
    pub fn new(s: I, e: I) -> Self {
        Self { start: s, end: e }
    }

    pub fn compare_other(&self, other: &Self) -> OverlapOrdering {
        match (
            self.start.partial_cmp(&other.end),
            self.end.partial_cmp(&other.start),
        ) {
            (None, None) => OverlapOrdering::NoOverlap,
            (None, Some(_)) => OverlapOrdering::NoOverlap,
            (Some(_), None) => OverlapOrdering::NoOverlap,
            (Some(se), Some(es)) => match (se, es) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => OverlapOrdering::Less,
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                    OverlapOrdering::OverlapEqualLess
                }
                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => OverlapOrdering::Equal,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                    OverlapOrdering::OverlapEqualGreater
                }
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                    OverlapOrdering::Greater
                }
                // SuperSet or SubSet
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                    match (
                        self.start.partial_cmp(&other.start),
                        self.end.partial_cmp(&other.end),
                    ) {
                        (None, None) => OverlapOrdering::NoOverlap,
                        (None, Some(_)) => OverlapOrdering::NoOverlap,
                        (Some(_), None) => OverlapOrdering::NoOverlap,
                        (Some(ss), Some(ee)) => match (ss, ee) {
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                                OverlapOrdering::SuperSet
                            }
                            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                                OverlapOrdering::SuperSet
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
                                OverlapOrdering::SubSet
                            }
                            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                                OverlapOrdering::SubSet
                            }
                            _ => {
                                println!("Should have matched before");
                                OverlapOrdering::NoOverlap
                            }
                        },
                    }
                }
                _ => {
                    println!("Not possible");
                    OverlapOrdering::NoOverlap
                }
            },
        }
    }

    pub fn compare_point(&self, other: &I) -> OverlapOrdering {
        match self.start.partial_cmp(&other) {
            Some(ord) => todo!(),
            None => OverlapOrdering::NoOverlap,
        }
    }
}
