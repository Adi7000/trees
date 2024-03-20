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
        let new_node = if let Some(root) = &self.root {
            let new_node = root.borrow_mut().binary_tree_insert(key.clone());
            new_node
        } else {
            let rc_node = TreeNode::new(Node::Avl(AvlNode{}), key.clone());
            self.root = Some(rc_node.clone());
            Some(rc_node)
        };

        //Re-Balance the tree starting at new node and then going up to all ancestors
        let mut current_node = new_node.clone();
        while let Some(ref rc_node) = current_node.clone() {
            let balance_factor: i64 = get_balance_factor(&rc_node.borrow()); //.clone()?
            //println!("{}:{}:{}", balance_factor, node.key, key);
            if balance_factor > 1 && key < rc_node.borrow().left_child.clone().unwrap().borrow().key {
                self.set_root_after_rotate(rc_node.borrow_mut().right_rotate());
            } else if balance_factor < -1 && key > rc_node.borrow().right_child.clone().unwrap().borrow().key {
                //println!("1");
                self.set_root_after_rotate(rc_node.borrow_mut().left_rotate());
            } else if balance_factor > 1 && key > rc_node.borrow().left_child.clone().unwrap().borrow().key {
                let rc_left_child = rc_node.borrow().left_child.clone().unwrap();
                self.set_root_after_rotate(rc_left_child.borrow_mut().left_rotate());
                self.set_root_after_rotate(rc_node.borrow_mut().right_rotate());
            } else if balance_factor < -1 && key < rc_node.borrow().right_child.clone().unwrap().borrow().key {
                let rc_right_child = rc_node.borrow().right_child.clone().unwrap();
                self.set_root_after_rotate(rc_right_child.borrow_mut().right_rotate());
                self.set_root_after_rotate(rc_node.borrow_mut().left_rotate());
            } else {
                rc_node.borrow_mut().fix_height();
            }

            //update current node
            current_node = rc_node.borrow().parent.clone();
        };

        new_node
    }

    fn set_root_after_rotate(&mut self, new_root: Option<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(new_node) = new_root {
            self.root = Some(new_node);
        };
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

pub fn get_balance_factor<T>(node: &TreeNode<T>) -> i64 {
    let mut r_height: u32 = 0;
    let mut l_height: u32 = 0;
    if let Some(ref r_node) = node.right_child {
        r_height = r_node.borrow().height + 1
    }
    if let Some(ref l_node) = node.left_child {
        l_height = l_node.borrow().height + 1
    }
    i64::from(l_height) - i64::from(r_height)
}