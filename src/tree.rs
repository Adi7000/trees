use crate::avl_tree::{self, AvlNode};
use crate::red_black_tree::RedBlackNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/*Lets try not to change this file too much unless a method is being implemebted
If you really need to change the structure please discuuss with the team first */

#[derive(Debug, Clone, Copy)]
pub enum Node {
    Avl(AvlNode),
    RedBlack(RedBlackNode),
}

// pub struct TreeNode<T> {
//     pub key: T,
//     pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
//     left: Option<Rc<RefCell<TreeNode<T>>>>,
//     right: Option<Rc<RefCell<TreeNode<T>>>>,
//     kind: Node,
// }

#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    pub key: T,
    pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub left_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right_child: Option<Rc<RefCell<TreeNode<T>>>>,
    pub root: Option<Rc<RefCell<TreeNode<T>>>>,
    pub kind: Node,
    pub height: u32, //number of edges from the furthest down leaf node
}

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> TreeNode<T> {
    pub fn new(node: Node, key: T) -> Rc<RefCell<TreeNode<T>>> {
        let ptr_node: Rc<RefCell<TreeNode<T>>> = Rc::new(RefCell::new(TreeNode {
            kind: node,
            key: key,
            root: None,
            parent: None,
            left_child: None,
            right_child: None,
            height: 0,
        }));
        ptr_node.borrow_mut().root = Some(ptr_node.clone());
        ptr_node
    }
    pub fn binary_tree_insert(&mut self, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let rc_current_node = self.root.clone().unwrap();
        // DONT NEED TO INSERT IF KEY PRESENT
        if self.key == key {
            return None;
        }

        let inserted_node = if key < self.key {
            // WILL INSERT ON LEFT SUBTREE

            // IF WE ARE CURRENTLY AT LEAF (I.E. NONE), INSERT
            if self.left_child.is_none() {
                let new_node = TreeNode::new(self.kind, key.clone());
                new_node.borrow_mut().parent = Some(Rc::clone(&rc_current_node));
                self.left_child = Some(new_node.clone());
                self.height = std::cmp::max::<u32>(self.height, 1);
                //FIXME recolor(&rc_current_node.borrow().left); // REBALANCE TREE
                Some(new_node)
            } else {
                // RECURSIVE STEP
                let inserted_node: Option<Rc<RefCell<TreeNode<T>>>> = self
                    .left_child
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .binary_tree_insert(key.clone());
                self.height = std::cmp::max::<u32>(
                    self.height,
                    self.left_child.clone().unwrap().borrow_mut().height + 1,
                );
                inserted_node
            }
        } else {
            // WILL INSERT ON RIGHT SUBTREE
            if self.right_child.is_none() {
                let new_node = TreeNode::new(self.kind, key.clone());
                new_node.borrow_mut().parent = Some(Rc::clone(&rc_current_node));
                self.right_child = Some(new_node.clone());
                self.height = std::cmp::max::<u32>(self.height, 1);
                //FIXME recolor(&rc_current_node.borrow().right); // REBALANCE TREE
                Some(new_node)
            } else {
                // RECURSIVE STEP
                let inserted_node: Option<Rc<RefCell<TreeNode<T>>>> = self
                    .right_child
                    .clone()
                    .unwrap()
                    .borrow_mut()
                    .binary_tree_insert(key.clone());
                self.height = std::cmp::max::<u32>(
                    self.height,
                    self.right_child.clone().unwrap().borrow_mut().height + 1,
                );
                inserted_node
            }
        };


        inserted_node
    }
    /** fixes the height feild of a node and all its ancestors
     *
     */
    fn fix_height_up(&mut self, borrowed_childs_height: Option<(u32, Rc<RefCell<TreeNode<T>>>)>) {
        //Fix current node
        let mut r_height: u32 = 0;
        let mut l_height: u32 = 0;

        //get height from right child
        if let Some(ref r_node) = self.right_child {
            //If this child is already borrowed in outerscope, get its height from outside scope
            if let Some((borrowed_height, borrowed_pointer)) = borrowed_childs_height.clone() {
                if Rc::ptr_eq(r_node, &borrowed_pointer) {
                    r_height = borrowed_height;
                } else {
                    r_height = r_node.borrow().height;
                }
            } else {
                r_height = r_node.borrow().height;
            }
            r_height += 1;
        }
        //get height form left child
        if let Some(ref l_node) = self.left_child {
            if let Some((borrowed_height, borrowed_pointer)) = borrowed_childs_height {
                if Rc::ptr_eq(l_node, &borrowed_pointer) {
                    l_height = borrowed_height;
                } else {
                    l_height = l_node.borrow().height;
                }
            } else {
                l_height = l_node.borrow().height;
            }
            l_height += 1;
        }
        self.height = std::cmp::max::<u32>(l_height, r_height);

        //Fix ancestors
        if let Some(ref parent_node) = self.parent {
            parent_node
                .borrow_mut()
                .fix_height_up(Some((self.height, self.root.clone().unwrap())));
        }
    }
     /** fixes the height of a node and its parent (parent must exist)
     *
     */
    fn fix_height_self_and_parent(&mut self) {

        //fix current node
        self.fix_height();

        // fix parent node's height (cannot call fix_height on parent because this node is a child and is already borrowed)
        let mut r_height: u32 = 0;
        let mut l_height: u32 = 0;

        let rc_parent_node = self.parent.clone().unwrap();
        let mut parent_node = rc_parent_node.borrow_mut();

        //get height from parent's right child
        if let Some(ref r_node) = parent_node.right_child {
            //If this child is self, get its height from self
            if Rc::ptr_eq(r_node, &self.root.clone().unwrap()) {
                r_height = self.height;
            } else {
                r_height = r_node.borrow().height;
            }
            r_height += 1;
        }
        //get height form left child
        if let Some(ref l_node) = parent_node.left_child {
            if Rc::ptr_eq(l_node, &self.root.clone().unwrap()) {
                l_height = self.height;
            } else {
                l_height = l_node.borrow().height;
            }
            l_height += 1;
        }
        parent_node.height = std::cmp::max::<u32>(l_height, r_height);

        
    }
    /** fixes height of current node by reading children */
    pub fn fix_height(&mut self) {

        //fix current node
        let mut r_height: u32 = 0;
        let mut l_height: u32 = 0;
        if let Some(ref r_node) = self.right_child {
            r_height = r_node.borrow().height + 1;
        }
        if let Some(ref l_node) = self.left_child {
            l_height = l_node.borrow().height + 1;
        }
        self.height = std::cmp::max::<u32>(l_height, r_height);

    }
    /**
     * returns the new root node of the tree if the root node is changed while rotating
     * This is returned to proplerly update the root node of a tree
     */
    pub fn left_rotate(&mut self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        // Note all terminology is relative to the initial tree configuration

        let right_child = self
            .right_child
            .take()
            .expect("Node must have right child to rotate");
        let root = self.root.clone().unwrap();

        let return_node;
        if self.parent.is_none() {
            //this node is root node of tree
            return_node = Some(right_child.clone());
        } else {
            return_node = None;
        }

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

        //fix height field of all nodes up
        self.fix_height_self_and_parent();

        //TODO Here is how I plan to handle node specific changes liek color
        //Feel free to move this block to start
        match self.kind {
            Node::Avl(_) => {}
            Node::RedBlack(_) => {}
        }

        return_node
    }

    /**
     * returns the new root node of the tree if the root node is changed while rotating
     * This is returned to proplerly update the root node of a tree
     */
    pub fn right_rotate(&mut self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        // Note all terminology is relative to the initial tree configuration

        let left_child = self
            .left_child
            .take()
            .expect("Node must have left child to rotate");
        let root = self.root.clone().unwrap();

        let return_node;
        if self.parent.is_none() {
            //this node is root node of tree
            return_node = Some(left_child.clone());
        } else {
            return_node = None;
        }

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

        //fix height field of all nodes up
        self.fix_height_self_and_parent();

        //TODO Here is how I plan to handle node specific changes liek color
        //Feel free to move this block to start
        match self.kind {
            Node::Avl(_) => {}
            Node::RedBlack(_) => {}
        }
        return_node
    }

    pub fn print_in_order_traverse(&mut self) {
        if let Some(l_node) = self.left_child.take() {
            self.left_child = Some(l_node.clone());
            l_node.borrow_mut().print_in_order_traverse();
        }
        match self.kind {
            Node::Avl(_) => println!("(Key: {:#?}, Height: {:#?})", self.key, self.height),
            Node::RedBlack(node) => println!(
                "(Key: {:#?}, Height: {:#?}, Color: {:#?})",
                self.key, self.height, node.color
            ),
        }
        if let Some(r_node) = self.right_child.take() {
            self.right_child = Some(r_node.clone());
            r_node.borrow_mut().print_in_order_traverse();
        }
    }

    pub fn print_tree(&self) {
        // let indent: usize = (self.height * 8).try_into().unwrap();
        // print root
        println!("{}", self.key);
        // root's children
        let mut node_queue = self.get_children();
        for i in 0..self.height {
            // get children of children
            let mut next_queue = VecDeque::new();
            // print current children and get next children
            for node in &node_queue {
                print!("{} ", node.borrow().key);
                let children = node.borrow().get_children();
                for child in children {
                    next_queue.push_back(child);
                }
            }
            node_queue = next_queue;
            println!();
        }
    }

    fn get_children(&self) -> VecDeque<Rc<RefCell<TreeNode<T>>>> {
        let mut node_queue = VecDeque::new();
        // let mut print_queue = VecDeque::new();
        if let Some(lc_node) = &self.left_child {
            node_queue.push_back(lc_node.clone());
        }
        if let Some(rc_node) = &self.right_child {
            node_queue.push_back(rc_node.clone());
        }
        node_queue
    }

    // fn print_recursive(&self, depth: usize) {
    //     let indent = 12;
    //     if let Some(lc_node) = &self.left_child {
    //         let left_indent = indent - 4;
    //         print!(
    //             "{:indent$}{}",
    //             "",
    //             lc_node.borrow().key,
    //             indent = left_indent
    //         );
    //         lc_node.borrow().print_recursive(left_indent);
    //     }
    //     if let Some(rc_node) = &self.right_child {
    //         let right_indent = indent + 4;
    //         print!(
    //             "{:indent$}{}",
    //             "",
    //             rc_node.borrow().key,
    //             indent = right_indent
    //         );
    //         rc_node.borrow().print_recursive(right_indent);
    //     }
    //     println!();
    // }
}

// impl<T> TreeNode<T> {
//     pub fn binary_tree_insert(self, data:T) {
//     }
//     pub fn rotate_nodes(self) {
//     }
// }
