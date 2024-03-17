use crate::avl_tree::AvlNode;
use crate::red_black_tree::RedBlackNode;
use std::cell::RefCell;
use std::rc::Rc;


/*Lets try not to change this file too much unless a method is being implemebted
If you really need to change the structure please discuuss with the team first */

enum Node {
    Avl(AvlNode),
    RedBlack(RedBlackNode)
}
pub struct TreeNode<T> {
    pub key: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
    kind: Node,
}

impl<T> TreeNode<T> {
    pub fn binary_tree_insert(self, data:T) {
        // binary tree insertion here
        if self.key == key {
            return;
        }

        let temp = if key < self.key {
            &mut self.left_child
        } else {
            &mut self.right_child
        };

        match temp {
            Some(node) => {
                node.borrow_mut().binary_tree_insert(key.clone());
            }
            None => {
                // TODO: THIS LINE NEEDS CHANGING
                let mut new_node = TreeNode::new_red_black(key.clone());
                
                match &self.parent.as_ref() {
                    Some(x) => {
                        new_node.parent = Some(Rc::clone(&self.parent.as_ref().unwrap()));
                    }
                    None => {
                        new_node.parent = None
                    }
                }
                
                if key < self.key {
                    self.left_child = Some(Rc::new(RefCell::new(new_node)))
                } else {
                    self.right_child = Some(Rc::new(RefCell::new(new_node)))
                };
            }
        }
    }
    pub fn rotate_nodes(self) {

    }
}

