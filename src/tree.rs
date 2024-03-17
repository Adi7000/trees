use crate::avl_tree::AvlNode;
use crate::red_black_tree::RedBlackNode;
use std::cell::RefCell;
use std::rc::Rc;


/*Lets try not to change this file too much unless a method is being implemebted
If you really need to change the structure please discuuss with the team first */

#[derive(Debug)]
pub enum Node {
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
        // DONT NEED TO INSERT IF KEY PRESENT
        if self.key == key {
            return;
        }

        // TRAVERSE THE TREE BY ONE STEP
        let temp = if key < self.key {
            &mut self.left_child
        } else {
            &mut self.right_child
        };

        match temp {
            Some(fahrin) => {
                // RECURSIVE STEP
                fahrin.borrow_mut().binary_tree_insert(key.clone());
            }
            None => {
                // NEEDS CHANGING MAYBE
                let mut new_node = TreeNode::new_red_black(key.clone());

                // SETTING PARENT FIELD
                match &self.parent.as_ref() {
                    Some(x) => {
                        new_node.parent = Some(Rc::clone(&self.parent.as_ref().unwrap()));
                    }
                    None => {
                        new_node.parent = None
                    }
                }

                // INSERTING NEW NODE
                if key < self.key {
                    self.left_child = Some(Rc::new(RefCell::new(new_node)))
                } else {
                    self.right_child = Some(Rc::new(RefCell::new(new_node)))
                };
            }
        }
    }

    pub fn left_rotate(&mut self) {
        // Note all terminology is relative to the initial tree configuration

        let right_child = self.right_child.take().expect("Node must have right child to rotate");
        let root = right_child.borrow_mut().parent.take().unwrap();  //same as self but is smart pointer

        // Connect parrent (or None) and right child
        if let Some(parent) = self.parent.take() {
            right_child.borrow_mut().parent = Some(parent.clone());
            if parent.borrow().key > right_child.borrow().key {
                parent.borrow_mut().left_child = Some(right_child.clone());
            } else {
                parent.borrow_mut().right_child = Some(right_child.clone());
            }
        } else {
            right_child.borrow_mut().parent = None;
        }

        // Connect right child's left child (or none) and root
        if let Some(right_childs_left_child) = right_child.borrow_mut().left_child.take() {
            right_childs_left_child.borrow_mut().parent = Some(root.clone());
            self.right_child = Some(right_childs_left_child.clone());
        } else {
            self.right_child = None;
        }

        // Reconnect right child to root
        right_child.borrow_mut().left_child = Some(root.clone());
        self.parent = Some(right_child.clone());

        //TODO Here is how I plan to handle node specific changes liek color
        //Feel free to move this block to start 
        match self.kind {
            Node::Avl(_) => {},
            Node::RedBlack(_) => {},
        }

    }

    pub fn right_rotate(&mut self) {
        // Note all terminology is relative to the initial tree configuration

        let left_child = self.left_child.take().expect("Node must have left child to rotate");
        let root = left_child.borrow_mut().parent.take().unwrap(); //same as self but is smart pointer

        // Connect parrent (or None) and left child
        if let Some(parent) = self.parent.take() {
            left_child.borrow_mut().parent = Some(parent.clone());
            if parent.borrow().key > left_child.borrow().key {
                parent.borrow_mut().left_child = Some(left_child.clone());
            } else {
                parent.borrow_mut().right_child = Some(left_child.clone());
            }
        } else {
            left_child.borrow_mut().parent = None;
        }

        // Connect left child's right child (or none) and root
        if let Some(left_childs_right_child) = left_child.borrow_mut().right_child.take() {
            left_childs_right_child.borrow_mut().parent = Some(root.clone());
            self.left_child = Some(left_childs_right_child.clone());
        } else {
            self.left_child = None;
        }

        // Reconnect left child to root
        left_child.borrow_mut().right_child = Some(root.clone());
        self.parent = Some(left_child.clone());

        //TODO Here is how I plan to handle node specific changes liek color
        //Feel free to move this block to start 
        match self.kind {
            Node::Avl(_) => {},
            Node::RedBlack(_) => {},
        }
    }
}



// impl<T> TreeNode<T> {
//     pub fn binary_tree_insert(self, data:T) {
//     }
//     pub fn rotate_nodes(self) {
//     }
// }

