use std::{cell::RefCell, rc::Rc};

use crate::InnerInfo;

use super::{CenteredIntervalTree, Node};

#[test]
fn create_empty() {
    let root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();

    assert_eq!(root.inner, None);
    assert_eq!(root.height(), 0);
}

#[test]
fn add_root_0() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add((0, 9), ());

    assert_eq!(root.height(), 1);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (0, 9)
            },
            left: None,
            center: None,
            right: None
        })))
    );
}

#[test]
fn add_new_root_0() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add((0, 9), ());
    root.add((-5, 15), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (-5, 15)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (0, 9)
                },
                left: None,
                center: None,
                right: None
            }))),
            right: None
        })))
    );
}

#[test]
fn add_left_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((0, 4), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (0, 4)
                },
                left: None,
                center: None,
                right: None
            }))),
            center: None,
            right: None
        })))
    );
}

#[test]
fn add_right_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((10, 14), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: None,
            center: None,
            right: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (10, 14)
                },
                left: None,
                center: None,
                right: None
            }))),
        })))
    );
}

#[test]
fn add_center_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((6, 8), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (6, 8)
                },
                left: None,
                center: None,
                right: None
            }))),
            right: None,
        })))
    );
}

#[test]
fn add_left_then_left_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((2, 4), ());
    root.add((0, 1), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (2, 4)
                },
                left: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (0, 1)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                center: None,
                right: None
            }))),
            center: None,
            right: None,
        })))
    );
}

#[test]
fn add_left_then_center_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((2, 4), ());
    root.add((3, 4), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (2, 4)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (3, 4)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None
            }))),
            center: None,
            right: None,
        })))
    );
}

#[test]
fn add_left_then_right_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((1, 2), ());
    root.add((3, 4), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (1, 2)
                },
                left: None,
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (3, 4)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
            }))),
            center: None,
            right: None,
        })))
    );
}

#[test]
fn add_center_then_left_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 9), ());
    root.add((6, 8), ());
    root.add((4, 5), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 9)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (6, 8)
                },
                left: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (4, 5)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                center: None,
                right: None,
            }))),
            right: None,
        })))
    );
}

#[test]
fn add_center_then_center_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 10), ());
    root.add((6, 9), ());
    root.add((7, 8), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 2);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 10)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (6, 9)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (7, 8)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None,
            }))),
            right: None,
        })))
    );
}

#[test]
fn add_center_then_right_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 10), ());
    root.add((6, 8), ());
    root.add((9, 10), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 10)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (6, 8)
                },
                left: None,
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (9, 10)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
            }))),
            right: None,
        })))
    );
}

#[test]
fn add_right_then_left_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 9), ());
    root.add((12, 13), ());
    root.add((10, 11), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 9)
            },
            left: None,
            center: None,
            right: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (12, 13)
                },
                left: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (10, 11)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                center: None,
                right: None,
            }))),
        })))
    );
}

#[test]
fn add_right_then_center_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 9), ());
    root.add((12, 15), ());
    root.add((13, 14), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 9)
            },
            left: None,
            center: None,
            right: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (12, 15)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (13, 14)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None,
            }))),
        })))
    );
}

#[test]
fn add_right_then_right_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((4, 10), ());
    root.add((12, 15), ());
    root.add((16, 17), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (4, 10)
            },
            left: None,
            center: None,
            right: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (12, 15)
                },
                left: None,
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (16, 17)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
            }))),
        })))
    );
}

#[test]
fn add_left_then_bigger_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((2, 3), ());
    root.add((1, 4), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (1, 4)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (2, 3)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None
            }))),
            center: None,
            right: None,
        })))
    );
}

#[test]
fn add_center_then_bigger_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 10), ());
    root.add((7, 8), ());
    root.add((6, 9), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 2);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 10)
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (6, 9)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (7, 8)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None
            }))),
            right: None,
        })))
    );
}

#[test]
fn add_right_then_bigger_0() {
    let mut root = CenteredIntervalTree::<i32, ()>::new();
    root.add((5, 9), ());
    root.add((12, 13), ());
    root.add((11, 14), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: (5, 9)
            },
            left: None,
            center: None,
            right: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: (),
                    interval: (11, 14)
                },
                left: None,
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: (),
                        interval: (12, 13)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                right: None
            }))),
        })))
    );
}

#[test]
fn search() {
    let mut root: CenteredIntervalTree<i32, String> = CenteredIntervalTree::new();
    assert_eq!(root.search(8).len(), 0);

    root.add((1, 9), String::from("Hello"));
    assert_eq!(root.search(8).len(), 1);
    assert_eq!(root.search(0).len(), 0);

    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: String::from("Hello"),
                interval: (1, 9)
            },
            left: None,
            center: None,
            right: None
        })))
    );

    root.add((-1, 0), String::from("left"));
    root.add((5, 8), String::from("center"));
    root.add((10, 11), String::from("right"));

    let result = root.search(8);
    assert_eq!(
        result,
        vec![
            InnerInfo {
                interval: (5, 8),
                value: String::from("center")
            },
            InnerInfo {
                interval: (1, 9),
                value: String::from("Hello")
            }
        ]
    );

    assert_ne!(
        result,
        vec![
            InnerInfo {
                interval: (1, 9),
                value: String::from("Hello")
            },
            InnerInfo {
                interval: (5, 8),
                value: String::from("center")
            }
        ]
    );
}

#[test]
fn iter_empty() {
    let root1: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();

    let mut iter = root1.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut root2: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root2.add((1, 4), ());
    root2.add((-1, 0), ());
    root2.add((1, 3), ());
    root2.add((5, 9), ());
    root2.add((1, 2), ());

    // (-1, 0) (1, 4) (5, 9)
    //         (1, 3)
    //         (1, 2)

    let mut iter = root2.iter();
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                value: (),
                interval: (1, 4)
            },
            0
        ))
    );
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                value: (),
                interval: (-1, 0)
            },
            0
        ))
    );
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                value: (),
                interval: (1, 3)
            },
            1
        ))
    );
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                value: (),
                interval: (1, 2)
            },
            2
        ))
    );
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                value: (),
                interval: (5, 9)
            },
            0
        ))
    );
}

use chrono::NaiveTime;

fn tree_iter_naive_time() {
    let mut root: CenteredIntervalTree<NaiveTime, String> = CenteredIntervalTree::new();
    root.add(
        (
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
        ),
        String::from("First"),
    );
    root.add(
        (
            NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
        ),
        String::from("Second"),
    );

    let mut iter = root.iter();
    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                interval: (
                    NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
                ),
                value: String::from("First")
            },
            0
        ))
    );

    assert_eq!(
        iter.next(),
        Some((
            InnerInfo {
                interval: (
                    NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                ),
                value: String::from("Second")
            },
            0
        ))
    );
}

#[test]
fn overlaps() {
    let mut root: CenteredIntervalTree<i32, String> = CenteredIntervalTree::new();
    root.add((1, 2), String::from("Node1"));
    root.add((3, 4), String::from("Node2"));
    root.add((5, 6), String::from("Node3"));

    root.add((1, 2), String::from("Node1"));
    root.add((5, 6), String::from("Node3"));
    root.add((7, 10), String::from("Node3"));

    root.add((5, 10), String::from("Node3"));

    root.add((6, 7), String::from("Node3"));

    // (1, 2) (3, 4) (5, 6) (7, 10)
    // (1, 2)        (5, 6)
    //               (5, 10)
    //               (6, 7)

    assert_eq!(root.height(), 6);
    assert_eq!(root.overlaps(), 3);
}

#[test]
fn one_big_overlaps_with_two_small() {
    let mut root: CenteredIntervalTree<i32, String> = CenteredIntervalTree::new();
    root.add((1, 2), String::from("Node1"));
    root.add((3, 4), String::from("Node2"));
    root.add((0, 10), String::from("Node3"));

    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                interval: (0, 10),
                value: String::from("Node3"),
            },
            left: None,
            center: Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    interval: (1, 2),
                    value: String::from("Node1"),
                },
                left: None,
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        interval: (3, 4),
                        value: String::from("Node2"),
                    },
                    left: None,
                    center: None,
                    right: None,
                }))),
            }))),
            right: None,
        })))
    )
}
