use std::{cell::RefCell, rc::Rc};

use crate::InnerInfo;
use crate::Interval;

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
    root.add(interval!([0, 9]), ());

    assert_eq!(root.height(), 1);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: (),
                interval: interval!([0, 9]),
            },
            left: None,
            center: None,
            right: None
        })))
    );
}
