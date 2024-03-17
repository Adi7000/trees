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
// pub struct TreeNode<T> {
//     pub key: T,
//     pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
//     left: Option<Rc<RefCell<TreeNode<T>>>>,
//     right: Option<Rc<RefCell<TreeNode<T>>>>,
//     kind: Node,
// }

#[derive(Debug)]
pub struct TreeNode<T> {
    pub key: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub left_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub kind: Node,
}

impl<T: Ord + Clone> TreeNode<T> {
    pub fn binary_tree_insert(&mut self, key:T) {
        if self.key == key {
            return;
        }

        let temp = if key < self.key {
            &mut self.left_child
        } else {
            &mut self.right_child
        };

        match temp {
            Some(fahrin) => {
                fahrin.borrow_mut().binary_tree_insert(key.clone());
            }
            None => {

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


// impl<T> TreeNode<T> {
//     pub fn binary_tree_insert(self, data:T) {
//     }
//     pub fn rotate_nodes(self) {
//     }
// }

