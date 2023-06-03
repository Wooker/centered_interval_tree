use std::{cell::RefCell, fmt::Debug, rc::Rc};

/// Centered interval tree.
#[derive(Debug)]
pub struct CenTreeNode<I, V> {
    inner: Link<I, V>,
}

type Link<I, V> = Option<Rc<RefCell<Node<I, V>>>>;

#[derive(PartialEq, Debug, Clone)]
pub struct InnerInfo<I, V> {
    value: V,
    interval: (I, I),
}

impl<I, V> InnerInfo<I, V> {
    pub fn interval(&self) -> &(I, I) {
        &self.interval
    }

    pub fn value(&self) -> &V {
        &self.value
    }
}

#[derive(PartialEq, Debug)]
pub struct Node<I, V> {
    info: InnerInfo<I, V>,
    center: Link<I, V>,
    right: Link<I, V>,
}

impl<I, V> CenTreeNode<I, V>
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

    pub fn add(&mut self, interval: (I, I), value: V) {
        assert!(interval.0 < interval.1);

        let new_node = Rc::new(RefCell::new(Node {
            info: InnerInfo { interval, value },
            center: None,
            right: None,
        }));

        let interval_borrowed = new_node.borrow().info.interval.clone();

        // Root node is is Some or None
        if let Some(root) = self.inner.take() {
            // New interval is strictly less than the root interval or not
            // If so, create new node and place the root node to the right of it
            if interval_borrowed.0 < root.borrow().info.interval.0
                && interval_borrowed.1 < root.borrow().info.interval.0
            {
                let new_root = new_node.clone();
                new_root.borrow_mut().right = Some(root);
                self.inner = Some(new_root);
            } else {
                self.inner = Some(root);
                let mut current = self.inner.clone();

                // Else, move node by node
                while let Some(node) = current {
                    // If interval is strictly greater than the current node
                    if interval_borrowed.0 > node.borrow().info.interval.1
                        && interval_borrowed.1 > node.borrow().info.interval.1
                    {
                        // If right of the node is None - add new node
                        if node.borrow().right.is_none() {
                            node.borrow_mut().right = Some(new_node.clone());
                            break;
                        } else {
                            // Else check the right node of the right node...
                            if let Some(right_node) = node.borrow().right.as_ref() {
                                if interval_borrowed.0 < right_node.borrow().info.interval.0
                                    && interval_borrowed.0 < right_node.borrow().info.interval.1
                                {
                                    new_node.borrow_mut().right = Some(right_node.clone());
                                }
                            }
                            // Update link
                            if new_node.borrow().right.is_some() {
                                node.borrow_mut().right = Some(new_node.clone());
                                break;
                            }
                            current = node.borrow().right.clone();
                        }
                    } else {
                        if node.borrow().center.is_none() {
                            node.borrow_mut().center = Some(new_node.clone());
                            break;
                        } else {
                            current = node.borrow().center.clone();
                        }
                    }
                }
            }
        } else {
            self.inner = Some(new_node);
        }
    }

    pub fn remove(&self, _info: InnerInfo<I, V>) {
        todo!()
    }

    pub fn height(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(root) => {
                let center = Self::from_node(root.borrow_mut().center.clone());
                let center_height = center.height();

                let right = Self::from_node(root.borrow_mut().right.clone());
                let right_height = right.height();
                right_height.max(center_height) + 1
            }
        }
    }

    pub fn overlaps(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(root) => {
                let inner_right = Self::from_node(root.borrow().right.clone());
                let right_overlaps = inner_right.overlaps();

                match &root.borrow().center {
                    None => right_overlaps,
                    Some(center) => {
                        let inner_center = Self::from_node(Some(center.clone()));
                        let center_overlaps = inner_center.overlaps();

                        right_overlaps.max(center_overlaps + 1)
                    }
                }
            }
        }
    }

    pub fn search(&self, point: I) -> Vec<InnerInfo<I, V>> {
        match &self.inner {
            None => vec![],
            Some(root)
                if root.borrow().info.interval.0 < point
                    && root.borrow().info.interval.1 < point =>
            {
                let right = Self::from_node(root.borrow().right.clone());
                let mut result = right.search(point);

                result.push(InnerInfo {
                    interval: root.borrow().info.interval.clone(),
                    value: root.borrow().info.value.clone(),
                });
                result
            }
            Some(root) => {
                let center = Self::from_node(root.borrow().center.clone());
                let mut result = center.search(point);

                result.push(InnerInfo {
                    interval: root.borrow().info.interval.clone(),
                    value: root.borrow().info.value.clone(),
                });
                result
            }
        }
    }
}

pub struct CenTreeNodeIterator<I, V> {
    stack: Vec<(Link<I, V>, usize)>,
}

impl<I, V> Iterator for CenTreeNodeIterator<I, V>
where
    I: Clone + Debug,
    V: Clone + Debug,
{
    type Item = (InnerInfo<I, V>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, layer)) = self.stack.pop() {
            let info = node.as_ref().unwrap().borrow().info.clone();

            if let Some(right) = node.as_ref().unwrap().borrow().right.as_ref() {
                self.stack.push((Some(Rc::clone(right)), layer));
            }

            if let Some(center) = node.as_ref().unwrap().borrow().center.as_ref() {
                self.stack.push((Some(Rc::clone(center)), layer + 1));
            }

            return Some((info, layer));
        }

        None
    }
}

impl<I, V> CenTreeNode<I, V>
where
    I: Debug,
    V: Debug,
{
    pub fn iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();

        if let Some(root) = self.inner.as_ref() {
            stack.push((Some(Rc::clone(root)), 0));
        }

        CenTreeNodeIterator { stack }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::InnerInfo;

    use super::{CenTreeNode, Node};

    #[test]
    fn create_and_add_after() {
        let mut root: CenTreeNode<i32, String> = CenTreeNode::new();
        assert_eq!(root.inner, None);

        root.add((1, 2), String::from("1"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("1"),
                    interval: (1, 2)
                },
                center: None,
                right: None
            })))
        );

        root.add((5, 6), String::from("3"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("1"),
                    interval: (1, 2)
                },
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("3"),
                        interval: (5, 6)
                    },
                    center: None,
                    right: None
                })))
            })))
        );

        root.add((3, 4), String::from("2"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("1"),
                    interval: (1, 2)
                },
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("2"),
                        interval: (3, 4)
                    },
                    center: None,
                    right: Some(Rc::new(RefCell::new(Node {
                        info: InnerInfo {
                            value: String::from("3"),
                            interval: (5, 6)
                        },
                        center: None,
                        right: None
                    })))
                })))
            })))
        );
    }

    #[test]
    fn create_and_add_before() {
        let mut root: CenTreeNode<i32, String> = CenTreeNode::new();
        assert_eq!(root.inner, None);

        root.add((1, 2), String::from("1"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("1"),
                    interval: (1, 2)
                },
                center: None,
                right: None
            })))
        );

        root.add((-1, 0), String::from("0"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("0"),
                    interval: (-1, 0)
                },
                center: None,
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("1"),
                        interval: (1, 2)
                    },
                    center: None,
                    right: None
                })))
            })))
        );

        root.add((0, 1), String::from("0.5"));
        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("0"),
                    interval: (-1, 0)
                },
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("0.5"),
                        interval: (0, 1)
                    },
                    center: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("1"),
                        interval: (1, 2)
                    },
                    center: None,
                    right: None
                })))
            })))
        );
    }

    #[test]
    fn search() {
        let mut root: CenTreeNode<i32, String> = CenTreeNode::new();
        root.add((1, 9), String::from("Hello"));

        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("Hello"),
                    interval: (1, 9)
                },
                center: None,
                right: None
            })))
        );

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
    fn iter() {
        let root1: CenTreeNode<i32, String> = CenTreeNode::new();

        let mut iter = root1.iter();
        assert_eq!(iter.next(), None);

        let mut root2: CenTreeNode<i32, String> = CenTreeNode::new();
        root2.add((1, 4), String::from("Node1"));
        root2.add((-1, 0), String::from("Node2"));
        root2.add((1, 3), String::from("Node3"));
        root2.add((5, 9), String::from("Node5"));
        root2.add((1, 2), String::from("Node4"));

        // (-1, 0)
        // (1, 4)  (1, 3) (1, 2)
        // (5, 9)

        let mut iter = root2.iter();
        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    value: String::from("Node2"),
                    interval: (-1, 0)
                },
                0
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    value: String::from("Node1"),
                    interval: (1, 4)
                },
                0
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    value: String::from("Node3"),
                    interval: (1, 3)
                },
                1
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    value: String::from("Node4"),
                    interval: (1, 2)
                },
                2
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    value: String::from("Node5"),
                    interval: (5, 9)
                },
                0
            ))
        );
    }

    use chrono::NaiveTime;

    #[test]
    fn tree_iter_naive_time() {
        let mut root: CenTreeNode<NaiveTime, String> = CenTreeNode::new();
        root.add(
            (
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            String::from("First"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
            ),
            String::from("Second"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(12, 15, 0).unwrap(),
                NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
            ),
            String::from("Fourth"),
        );

        // (12, 14) (12, 12.5) (12.25, 13.5)

        assert_eq!(root.overlaps(), 2);

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
                        NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                        NaiveTime::from_hms_opt(12, 30, 0).unwrap(),
                    ),
                    value: String::from("Second")
                },
                1
            ))
        );

        assert_eq!(
            iter.next(),
            Some((
                InnerInfo {
                    interval: (
                        NaiveTime::from_hms_opt(12, 15, 0).unwrap(),
                        NaiveTime::from_hms_opt(13, 30, 0).unwrap(),
                    ),
                    value: String::from("Fourth")
                },
                2
            ))
        );
    }

    #[test]
    fn overlaps() {
        let mut root: CenTreeNode<i32, String> = CenTreeNode::new();
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
}
