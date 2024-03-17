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
    }
    pub fn rotate_nodes(self) {

    }
}

