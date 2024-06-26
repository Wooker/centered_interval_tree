use std::cell::RefCell;
use std::rc::Rc;

use crate::inner_info::InnerInfo;

pub type Link<I, V> = Option<Rc<RefCell<Node<I, V>>>>;

#[derive(PartialEq, Debug, Clone)]
pub struct Node<I, V> {
    pub info: InnerInfo<I, V>,
    pub left: Link<I, V>,
    pub center: Link<I, V>,
    pub right: Link<I, V>,
}

#[allow(unused)]
impl<I, V> Node<I, V>
where
    I: PartialOrd,
{
    pub fn info(&self) -> &InnerInfo<I, V> {
        &self.info
    }
    pub fn left(&self) -> &Link<I, V> {
        &self.left
    }
    pub fn center(&self) -> &Link<I, V> {
        &self.center
    }
    pub fn right(&self) -> &Link<I, V> {
        &self.right
    }
}
