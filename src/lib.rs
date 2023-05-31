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

#[derive(PartialEq, Debug)]
pub struct Node<I, V> {
    info: InnerInfo<I, V>,
    left: Link<I, V>,
    center: Link<I, V>,
    right: Link<I, V>,
}

impl<I, V> CenTreeNode<I, V>
where
    I: PartialOrd + Clone + Debug,
    V: Clone,
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
                if interval.0 < root.borrow().info.interval.0
                    && interval.1 < root.borrow().info.interval.0
                {
                    let mut left = Self::from_node(root.borrow_mut().left.clone());
                    left.add(interval, value);
                    root.borrow_mut().left = left.inner;
                } else if interval.0 > root.borrow().info.interval.1
                    && interval.1 > root.borrow().info.interval.1
                {
                    let mut right = Self::from_node(root.borrow_mut().right.clone());
                    right.add(interval, value);
                    root.borrow_mut().right = right.inner;
                } else {
                    let mut center = Self::from_node(root.borrow_mut().center.clone());
                    center.add(interval, value);
                    root.borrow_mut().center = center.inner;
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

    pub fn search(&self, point: I) -> Vec<InnerInfo<I, V>> {
        match &self.inner {
            None => vec![],
            Some(root)
                if root.borrow().info.interval.0 > point
                    && root.borrow().info.interval.1 > point =>
            {
                let left = Self::from_node(root.borrow().left.clone());
                let mut result = left.search(point);

                result.push(InnerInfo {
                    interval: root.borrow().info.interval.clone(),
                    value: root.borrow().info.value.clone(),
                });
                result
            }
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
    stack: Vec<Link<I, V>>,
}

impl<I, V> Iterator for CenTreeNodeIterator<I, V>
where
    I: Clone,
    V: Clone,
{
    type Item = InnerInfo<I, V>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop().flatten() {
            let info = node.borrow().info.clone();

            if let Some(right) = node.borrow().right.as_ref() {
                self.stack.push(Some(Rc::clone(right)));
            }
            if let Some(center) = node.borrow().center.as_ref() {
                self.stack.push(Some(Rc::clone(center)));
            }
            if let Some(left) = node.borrow().left.as_ref() {
                self.stack.push(Some(Rc::clone(left)));
            }

            return Some(info);
        }

        None
    }
}

impl<I, V> CenTreeNode<I, V> {
    pub fn iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();
        if let Some(root) = self.inner.as_ref() {
            stack.push(Some(Rc::clone(root)));
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
    fn create_and_add() {
        let mut root: CenTreeNode<i32, String> = CenTreeNode::new();
        root.add((1, 9), String::from("Hello"));

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
        root.add((5, 10), String::from("center"));
        root.add((10, 11), String::from("right"));
        root.add((5, 7), String::from("center2"));
        root.add((10, 11), String::from("right2"));
        root.add((5, 6), String::from("center3"));
        root.add((5, 6), String::from("center4"));

        assert_eq!(
            root.inner,
            Some(Rc::new(RefCell::new(Node {
                info: InnerInfo {
                    value: String::from("Hello"),
                    interval: (1, 9)
                },
                left: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("left"),
                        interval: (-1, 0)
                    },
                    left: None,
                    center: None,
                    right: None
                }))),
                center: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("center"),
                        interval: (5, 10)
                    },
                    left: None,
                    center: Some(Rc::new(RefCell::new(Node {
                        info: InnerInfo {
                            value: String::from("center2"),
                            interval: (5, 7)
                        },
                        left: None,
                        center: Some(Rc::new(RefCell::new(Node {
                            info: InnerInfo {
                                value: String::from("center3"),
                                interval: (5, 6)
                            },
                            left: None,
                            center: Some(Rc::new(RefCell::new(Node {
                                info: InnerInfo {
                                    value: String::from("center4"),
                                    interval: (5, 6)
                                },
                                left: None,
                                center: None,
                                right: None
                            }))),
                            right: None
                        }))),
                        right: None
                    }))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(Node {
                    info: InnerInfo {
                        value: String::from("right"),
                        interval: (10, 11)
                    },
                    left: None,
                    center: Some(Rc::new(RefCell::new(Node {
                        info: InnerInfo {
                            value: String::from("right2"),
                            interval: (10, 11)
                        },
                        left: None,
                        center: None,
                        right: None
                    }))),
                    right: None,
                }))),
            })))
        );

        assert_eq!(root.height(), 5);
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

        let mut iter = root2.iter();
        assert_eq!(
            iter.next(),
            Some(InnerInfo {
                value: String::from("Node1"),
                interval: (1, 4)
            })
        );
        assert_eq!(
            iter.next(),
            Some(InnerInfo {
                value: String::from("Node2"),
                interval: (-1, 0)
            })
        );
        assert_eq!(
            iter.next(),
            Some(InnerInfo {
                value: String::from("Node3"),
                interval: (1, 3)
            })
        );
        assert_eq!(
            iter.next(),
            Some(InnerInfo {
                value: String::from("Node4"),
                interval: (1, 2)
            })
        );
        assert_eq!(
            iter.next(),
            Some(InnerInfo {
                value: String::from("Node5"),
                interval: (5, 9)
            })
        );
    }
}
