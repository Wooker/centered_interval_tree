use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    inner_info::InnerInfo,
    interval::{Interval, OverlapOrdering},
    iterator::CenTreeNodeIterator,
    node::{Link, Node},
};

macro_rules! node {
    ($val:expr, $int:expr, $left:expr, $center:expr, $right:expr) => {{
        Some(Rc::new(RefCell::new(Node {
            info: InnerInfo {
                value: $val,
                full_interval: $int.clone(),
                interval: $int,
            },
            left: $left,
            center: $center,
            right: $right,
        })))
    }};
}

/// Centered interval tree.
#[derive(Debug)]
pub struct CenteredIntervalTree<I, V>
where
    I: std::fmt::Debug,
{
    pub link: Link<I, V>,
}

#[allow(unused)]
impl<I, V> CenteredIntervalTree<I, V>
where
    I: PartialOrd + Clone + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { link: None }
    }

    pub fn from_node(node: Link<I, V>) -> Self {
        match node {
            None => Self { link: None },
            Some(n) => Self { link: Some(n) },
        }
    }

    pub fn add(&mut self, interval: Interval<I>, value: V) {
        println!("Adding {}, {:?}", interval, value);
        if let Some(root) = self.link.take() {
            let mut root_mut = root.borrow_mut();
            match root_mut.info.interval().compared_to(&interval) {
                OverlapOrdering::SuperSet => {
                    println!(
                        "\t{} is super of {}",
                        interval,
                        root.clone().borrow().info.interval,
                    );
                    let new_root = Rc::new(RefCell::new(root_mut.deref().to_owned()));
                    self.link = node!(value.clone(), interval.clone(), None, Some(new_root), None);
                }
                OverlapOrdering::Less => {
                    println!("\tAdding left {}", interval);
                    let mut left = Self::from_node(root_mut.left.clone());
                    left.add(interval, value);
                    root_mut.left = left.link;

                    let new_root = Rc::new(RefCell::new(root_mut.deref().to_owned()));
                    self.link = Some(new_root);
                }
                OverlapOrdering::Greater => {
                    println!("\tAdding right {}", interval);
                    let mut right = Self::from_node(root_mut.right.clone());
                    right.add(interval, value);
                    root_mut.right = right.link;

                    let new_root = Rc::new(RefCell::new(root_mut.deref().to_owned()));
                    self.link = Some(new_root);
                }
                OverlapOrdering::SubSet
                | OverlapOrdering::OverlapLess
                | OverlapOrdering::OverlapEqualLess
                | OverlapOrdering::Equal
                | OverlapOrdering::OverlapGreater
                | OverlapOrdering::OverlapEqualGreater => {
                    println!("\tAdding center {}", interval);
                    let mut center = Self::from_node(root_mut.center.clone());
                    center.add(interval, value);
                    root_mut.info.full_interval = root_mut.info.full_interval.clone()
                        + center
                            .link
                            .clone()
                            .unwrap()
                            .borrow()
                            .info
                            .full_interval
                            .clone();
                    println!(
                        "root({:?}) {} + center({:?}) {} , root full_int: {}",
                        root_mut.info.value,
                        root_mut.info.interval,
                        center.link.clone().unwrap().borrow().info.value,
                        center.link.clone().unwrap().borrow().info.interval,
                        root_mut.info.full_interval,
                    );
                    root_mut.center = center.link;
                    let new_root = Rc::new(RefCell::new(root_mut.deref().to_owned()));
                    self.link = Some(new_root);
                }
                OverlapOrdering::SuperSet => {
                    println!("\tSuper set {}", interval);
                }
                OverlapOrdering::NotPossible => panic!("Intervals are not defined"),
                _ => {
                    let new_root = Rc::new(RefCell::new(root_mut.deref().to_owned()));
                    self.link = Some(new_root);
                }
            };
        } else {
            println!("\tRoot is None, creating root");
            self.link = node!(value, interval, None, None, None);
            return;
        }
    }

    pub fn remove(&self, _info: InnerInfo<I, V>) {
        todo!()
    }

    pub fn height(&self) -> usize {
        match &self.link {
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
        match &self.link {
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
        match &self.link {
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

#[allow(unused)]
impl<I, V> CenteredIntervalTree<I, V>
where
    I: std::fmt::Debug,
{
    pub fn iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();

        if let Some(root) = self.link.as_ref() {
            stack.push((Some(Rc::clone(root)), 0, false));
        }

        CenTreeNodeIterator { stack }
    }
}
