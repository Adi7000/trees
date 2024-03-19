use std::{cell::RefCell, rc::Rc};

use crate::tree::*;


#[derive(Debug,Clone, Copy)]
pub struct AvlNode {
}

pub struct AvlTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree { root: None }
    }
    pub fn insert(&mut self, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        //FIXME Remove this return value (for debugging)
        if let Some(root) = &self.root {
            let new_node = root.borrow_mut().binary_tree_insert(key);
            //TODO Handle recoloring here
            new_node
        } else {
            let rc_node = TreeNode::new(Node::Avl(AvlNode{}), key);
            self.root = Some(rc_node.clone());
            Some(rc_node)
        }
    }

    // IN ORDER TRAVERSAL FUNCTION
    pub fn print_inorder(&mut self) {
        let root = self.root.take();
        match root {
            Some(node) => {
                self.root = Some(node.clone());
                node.borrow_mut().print_in_order_traverse();
            }
            None => {}
        }
    }

    pub fn print_tree(&mut self) {
        let root = self.root.take();
        match root {
            Some(node) => {
                self.root = Some(node.clone());
                node.borrow_mut().print_tree();
            }
            None => {}
        }
    }

    pub fn height(&self) -> u32 {
        self.root.clone().unwrap().borrow().height
    }
}