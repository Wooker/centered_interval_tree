use crate::inner_info::InnerInfo;
use crate::Link;

#[derive(PartialEq, Debug, Clone)]
pub struct Node<I, V>
where
    I: PartialOrd,
{
    pub(crate) info: InnerInfo<I, V>,
    pub(crate) left: Link<I, V>,
    pub(crate) center: Link<I, V>,
    pub(crate) right: Link<I, V>,
}

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
