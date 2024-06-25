use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[cfg(test)]
mod test;

pub mod inner_info;
use inner_info::InnerInfo;

pub mod interval;
use interval::Interval;

mod iterator;
use iterator::CenTreeNodeIterator;

use crate::interval::OverlapOrdering;

/// Centered interval tree.
#[derive(Debug)]
pub struct CenteredIntervalTree<I, V> {
    pub inner: Link<I, V>,
}

type Link<I, V> = Option<Rc<RefCell<Node<I, V>>>>;

#[derive(PartialEq, Debug)]
pub struct Node<I, V> {
    pub info: InnerInfo<I, V>,
    pub left: Link<I, V>,
    pub center: Link<I, V>,
    pub right: Link<I, V>,
}

impl<I, V> CenteredIntervalTree<I, V>
where
    I: PartialOrd + Clone + Debug,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self { inner: None }
    }

    fn from_node(node: Link<I, V>) -> Self {
        match node {
            None => Self { inner: None },
            Some(n) => Self { inner: Some(n) },
        }
    }

    pub fn add(&mut self, interval: Interval<I>, value: V) {
        if let Some(root) = self.inner.take() {
            match interval.compare_other(root.clone().borrow().info.interval()) {
                OverlapOrdering::SubSet => {
                    self.inner = Some(Rc::new(RefCell::new(Node {
                        info: InnerInfo {
                            value: value.clone(),
                            interval: interval.clone(),
                        },
                        left: None,
                        center: Some(root),
                        right: None,
                    })));
                    return;
                }
                _ => {
                    self.inner = Some(root);
                }
            };
        }

        match &self.inner {
            None => {
                self.inner = Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo { value, interval },
                    left: None,
                    center: None,
                    right: None,
                })))
            }
            Some(root) => {
                let mut root_mut = root.borrow_mut();
                match root_mut.info.interval().compare_other(&interval) {
                    OverlapOrdering::Less => {
                        // println!("{:?} is less than root, left now", interval);
                        let mut left = Self::from_node(root_mut.left.clone());
                        left.add(interval, value);
                        root_mut.left = left.inner;
                    }
                    OverlapOrdering::Greater => {
                        // println!("{:?} is greater than root, right now", interval);
                        let mut right = Self::from_node(root_mut.right.clone());
                        right.add(interval, value);
                        root_mut.right = right.inner;
                    }
                    OverlapOrdering::SubSet
                    | OverlapOrdering::OverlapLess
                    | OverlapOrdering::OverlapEqualLess
                    | OverlapOrdering::Equal
                    | OverlapOrdering::OverlapGreater
                    | OverlapOrdering::OverlapEqualGreater => {
                        // println!("{:?} overlaps the root, center now", interval);
                        let mut center = Self::from_node(root_mut.center.clone());
                        center.add(interval, value);
                        root_mut.center = center.inner;
                    }
                    _ => {
                        panic!(
                            "ADD: Unhandled case, {:?} to {:?}",
                            (interval, value),
                            self.inner
                        );
                    }
                }
            }
        }
    }

    pub fn remove(&self, _info: InnerInfo<I, V>) {
        todo!()
    }

    pub fn height(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(root) => {
                let left = Self::from_node(root.borrow_mut().left.clone());
                let left_height = left.height();

                let center = Self::from_node(root.borrow_mut().center.clone());
                let center_height = center.height();

                let right = Self::from_node(root.borrow_mut().right.clone());
                let right_height = right.height();
                left_height.max(right_height).max(center_height) + 1
            }
        }
    }

    pub fn overlaps(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(root) => {
                let inner_left = Self::from_node(root.borrow().left.clone());
                let left_overlaps = inner_left.overlaps();

                let inner_right = Self::from_node(root.borrow().right.clone());
                let right_overlaps = inner_right.overlaps();

                let max_left_right = left_overlaps.max(right_overlaps);

                match &root.borrow().center {
                    None => max_left_right,
                    Some(center) => {
                        let inner_center = Self::from_node(Some(center.clone()));
                        let center_overlaps = inner_center.overlaps();

                        max_left_right.max(center_overlaps + 1)
                    }
                }
            }
        }
    }

    pub fn search(&self, point: I) -> Vec<InnerInfo<I, V>> {
        match &self.inner {
            None => vec![],
            Some(root) => match root.borrow().info.interval.compare_point(&point) {
                _ => vec![],
            }, //     if root.borrow().info.interval > point && root.borrow().info.interval > point =>
               // {
               //     let left = Self::from_node(root.borrow().left.clone());
               //     left.search(point)
               // }
               // Some(root)
               //     if root.borrow().info.interval < point && root.borrow().info.interval < point =>
               // {
               //     let right = Self::from_node(root.borrow().right.clone());
               //     right.search(point)
               // }
               // Some(root) => {
               //     let center = Self::from_node(root.borrow().center.clone());
               //     let mut result = center.search(point);

               //     result.push(InnerInfo {
               //         interval: root.borrow().info.interval().clone(),
               //         value: root.borrow().info.value().clone(),
               //     });

               //     result
               // }
        }
    }
}

impl<I, V> CenteredIntervalTree<I, V> {
    pub fn iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();

        if let Some(root) = self.inner.as_ref() {
            stack.push((Some(Rc::clone(root)), 0, false));
        }

        CenTreeNodeIterator { stack }
    }
}
