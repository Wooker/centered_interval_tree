use std::{cell::RefCell, rc::Rc};

use crate::{inner_info::InnerInfo, interval::Interval, node::Node};

use super::CenteredIntervalTree;

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
    assert_eq!(root.inner, node!((), interval!([0, 9]), None, None, None));
}

#[test]
fn add_left_1() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([5, 9]), ());
    root.add(interval!([0, 4]), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        node!(
            (),
            interval!([5, 9]),
            node!((), interval!([0, 4]), None, None, None),
            None,
            None
        )
    );
}

#[test]
fn add_right_1() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([0, 4]), ());
    root.add(interval!([5, 9]), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.inner,
        node!(
            (),
            interval!([0, 4]),
            None,
            None,
            node!((), interval!([5, 9]), None, None, None)
        )
    );
}

#[test]
fn add_center_1() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([0, 9]), ());
    root.add(interval!([5, 6]), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 1);
    assert_eq!(
        root.inner,
        node!(
            (),
            interval!([0, 9]),
            None,
            node!((), interval!([5, 6]), None, None, None),
            None
        )
    );
}

#[test]
fn add_right_with_overlays() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([0, 9]), ());
    root.add(interval!([5, 13]), ());
    root.add(interval!([8, 18]), ());

    root.add(interval!([10, 15]), ());

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 2);
    assert_eq!(
        root.inner,
        node!(
            (),
            interval!([0, 9]),
            None,
            node!(
                (),
                interval!([5, 13]),
                None,
                node!((), interval!([8, 18]), None, None, None),
                None
            ),
            node!((), interval!([10, 15]), None, None, None)
        )
    );

    let right_cit = CenteredIntervalTree::from_node(root.inner.unwrap().borrow().right.clone());
    assert_eq!(right_cit.overlaps(), 0);
}
