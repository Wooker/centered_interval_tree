use crate::{inner_info::InnerInfo, node::Link};
use std::{fmt::Debug, rc::Rc};

pub struct CenTreeNodeIterator<I, V>
where
    I: std::fmt::Debug,
{
    pub(crate) stack: Vec<(Link<I, V>, usize, bool)>,
}

impl<I, V> Iterator for CenTreeNodeIterator<I, V>
where
    I: Clone + Debug,
    V: Clone + Debug,
{
    type Item = (InnerInfo<I, V>, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, layer, mut has_overlaps)) = self.stack.pop() {
            let info = node.as_ref().unwrap().borrow().info.clone();

            if let Some(right) = node.as_ref().unwrap().borrow().right.as_ref() {
                self.stack.push((Some(Rc::clone(right)), layer, false));
                // if CenteredIntervalTree::from_node(Some(right)).search(info.interval())
            }
            if let Some(center) = node.as_ref().unwrap().borrow().center.as_ref() {
                has_overlaps = true;
                self.stack.push((Some(Rc::clone(center)), layer + 1, false));
            }
            if let Some(left) = node.as_ref().unwrap().borrow().left.as_ref() {
                self.stack.push((Some(Rc::clone(left)), layer, false));
            }

            return Some((info, layer, has_overlaps));
        }

        None
    }
}
