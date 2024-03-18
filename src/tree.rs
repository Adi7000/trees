use crate::avl_tree::AvlNode;
use crate::red_black_tree::RedBlackNode;
use std::cell::RefCell;
use std::rc::Rc;


/*Lets try not to change this file too much unless a method is being implemebted
If you really need to change the structure please discuuss with the team first */

#[derive(Debug,Clone, Copy)]
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

#[derive(Debug,Clone)]
pub struct TreeNode<T> {
    pub key: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub left_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
    pub kind: Node,
}

impl<T: Ord + Clone + std::fmt::Debug> TreeNode<T> {
    pub fn new(node: Node, key:T) -> Rc<RefCell<TreeNode<T>>> {

        let ptr_node: Rc<RefCell<TreeNode<T>>>= Rc::new(RefCell::new(TreeNode{
            kind: node,
            key: key,
            root: None,
            parent: None,
            left_child: None,
            right_child: None,
        }));
        ptr_node.borrow_mut().root = Some(ptr_node.clone());
        ptr_node
    }
    pub fn binary_tree_insert(&mut self, key:T) -> Option<Rc<RefCell<TreeNode<T>>>>{
        let rc_current_node = self.root.clone().unwrap();
        // DONT NEED TO INSERT IF KEY PRESENT
        if self.key == key {
            return None;
        }
            
        if key < self.key {
            // WILL INSERT ON LEFT SUBTREE
            
            // IF WE ARE CURRENTLY AT LEAF (I.E. NONE), INSERT
            if self.left_child.is_none(){

                let new_node = TreeNode::new(self.kind, key);
                new_node.borrow_mut().parent = Some(Rc::clone(&rc_current_node));
                self.left_child = Some(new_node.clone());
                //FIXME recolor(&rc_current_node.borrow().left); // REBALANCE TREE
                Some(new_node)
            }
            
            else{
                // RECURSIVE STEP
                self.left_child.clone().unwrap().borrow_mut().binary_tree_insert(key)
            }
        } else {
            // WILL INSERT ON RIGHT SUBTREE
            if self.right_child.is_none(){
                let new_node = TreeNode::new(self.kind, key);
                new_node.borrow_mut().parent = Some(Rc::clone(&rc_current_node));
                self.right_child = Some(new_node.clone());
                //FIXME recolor(&rc_current_node.borrow().right); // REBALANCE TREE
                Some(new_node)
            }
            
            else{
                // RECURSIVE STEP
                self.right_child.clone().unwrap().borrow_mut().binary_tree_insert(key)
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

    pub fn print_in_order_traverse(&mut self) {
        if let Some(l_node) = self.left_child.take() {
            self.left_child = Some(l_node.clone());
            l_node.borrow_mut().print_in_order_traverse();
        }
        match self.kind {
            Node::Avl(_) => println!("(Key: {:#?})", self.key),
            Node::RedBlack(node) => println!("(Key: {:#?}, Color: {:#?})", self.key, node.color),
        }
        if let Some(r_node) = self.right_child.take() {
            self.right_child = Some(r_node.clone());
            r_node.borrow_mut().print_in_order_traverse();
        }
        
        
    }
}



// impl<T> TreeNode<T> {
//     pub fn binary_tree_insert(self, data:T) {
//     }
//     pub fn rotate_nodes(self) {
//     }
// }

