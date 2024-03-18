use crate::tree::*;
use std::{cell::RefCell, rc::Rc};


#[derive(Debug,Clone, Copy)]
pub struct AvlNode {
}

pub struct AvlTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>
}
impl<T: Ord + Clone> AvlTree<T> {
    pub fn insert(self, data: T) {
    }
}
