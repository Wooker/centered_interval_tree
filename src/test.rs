use std::{cell::RefCell, rc::Rc};

use crate::{inner_info::InnerInfo, interval::Interval, node::Node};

use super::CenteredIntervalTree;

#[test]
fn create_empty() {
    let root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();

    assert_eq!(root.link, None);
    assert_eq!(root.height(), 0);
}

#[test]
fn add_root_0() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([0, 9]), ());

    assert_eq!(root.height(), 1);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(root.link, node!((), interval!([0, 9]), None, None, None));
}

#[test]
fn add_left_1() {
    let mut root: CenteredIntervalTree<i32, ()> = CenteredIntervalTree::new();
    root.add(interval!([5, 9]), ());
    root.add(interval!([0, 4]), ());

    assert_eq!(root.height(), 2);
    assert_eq!(root.overlaps(), 0);
    assert_eq!(
        root.link,
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
        root.link,
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
        root.link,
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
    let mut root: CenteredIntervalTree<i32, usize> = CenteredIntervalTree::new();
    root.add(interval!([0, 9]), 1);
    root.add(interval!([5, 13]), 2);
    root.add(interval!([8, 18]), 3);

    root.add(interval!([10, 15]), 4);

    assert_eq!(root.height(), 3);
    assert_eq!(root.overlaps(), 2);
    assert_eq!(
        root.link,
        node!(
            1,
            interval!([0, 9]),
            None,
            node!(
                2,
                interval!([5, 13]),
                None,
                node!(3, interval!([8, 18]), None, None, None),
                None
            ),
            node!(4, interval!([10, 15]), None, None, None)
        )
    );

    let right_cit = CenteredIntervalTree::from_node(root.link.unwrap().borrow().right.clone());
    assert_eq!(right_cit.overlaps(), 0);
}

#[test]
fn full_interval_add_up() {
    let mut root: CenteredIntervalTree<i32, usize> = CenteredIntervalTree::new();
    root.add(interval!([1, 6]), 1);
    root.add(interval!([2, 7]), 2);
    root.add(interval!([3, 8]), 3);
    root.add(interval!([4, 9]), 4);

    assert_eq!(
        root.link.unwrap().borrow().info.full_interval,
        interval!([1, 9])
    );
}

#[test]
fn full_interval_add_down() {
    let mut root: CenteredIntervalTree<i32, usize> = CenteredIntervalTree::new();
    root.add(interval!([1, 6]), 1);
    root.add(interval!([2, 7]), 2);
    root.add(interval!([3, 8]), 3);
    root.add(interval!([4, 9]), 4);

    assert_eq!(
        root.link.unwrap().borrow().info.full_interval,
        interval!([1, 9])
    );
}

#[test]
fn full_interval_add_outside_inside() {
    let mut root: CenteredIntervalTree<i32, usize> = CenteredIntervalTree::new();
    root.add(interval!([4, 6]), 1);
    root.add(interval!([3, 7]), 2);
    root.add(interval!([2, 8]), 3);
    root.add(interval!([1, 9]), 4);

    assert_eq!(
        root.link.unwrap().borrow().info.full_interval,
        interval!([4, 6])
    );
}

#[test]
fn full_interval_add_inside_outside() {
    let mut root: CenteredIntervalTree<i32, usize> = CenteredIntervalTree::new();
    root.add(interval!([4, 6]), 1);
    root.add(interval!([3, 7]), 2);
    root.add(interval!([2, 8]), 3);
    root.add(interval!([1, 9]), 4);

    assert_eq!(
        root.link.unwrap().borrow().info.full_interval,
        interval!([1, 9])
    );
}
