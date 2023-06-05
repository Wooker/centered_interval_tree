use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[cfg(test)]
mod test;

/// Centered interval tree.
#[derive(Debug)]
pub struct CenteredIntervalTree<I, V> {
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
    left: Link<I, V>,
    center: Link<I, V>,
    right: Link<I, V>,
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

    pub fn add(&mut self, interval: (I, I), value: V) {
        assert!(interval.0 < interval.1);

        if let Some(root) = self.inner.take() {
            if interval.0 < root.borrow().info.interval.0
                && interval.1 > root.borrow().info.interval.1
            {
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
            } else {
                self.inner = Some(root);
            }
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
            if let Some(left) = node.as_ref().unwrap().borrow().left.as_ref() {
                self.stack.push((Some(Rc::clone(left)), layer));
            }

            if let Some(center) = node.as_ref().unwrap().borrow().center.as_ref() {
                self.stack.push((Some(Rc::clone(center)), layer + 1));
            }

            return Some((info, layer));
        }

        None
    }
}

impl<I, V> CenteredIntervalTree<I, V> {
    pub fn iter(&self) -> CenTreeNodeIterator<I, V> {
        let mut stack = Vec::new();

        if let Some(root) = self.inner.as_ref() {
            stack.push((Some(Rc::clone(root)), 0));
        }

        CenTreeNodeIterator { stack }
    }
}
