use crate::{inner_info::InnerInfo, Link};
use std::{fmt::Debug, rc::Rc};

pub struct CenTreeNodeIterator<I, V> {
    pub(crate) stack: Vec<(Link<I, V>, usize)>,
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
            if let Some(left) = node.as_ref().unwrap().borrow().left.as_ref() {
                self.stack.push((Some(Rc::clone(left)), layer));
            }

            return Some((info, layer));
        }

        None
    }
}
