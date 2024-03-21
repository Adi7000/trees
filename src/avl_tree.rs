use std::{cell::RefCell, rc::Rc};

use crate::tree::*;

#[derive(Debug, Clone, Copy)]
pub struct AvlNode {}

pub struct AvlTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree { root: None }
    }
    pub fn delete(&mut self, key:T) {
        // Case 2 leaf node with parent
        fn handle_case_2<T: PartialOrd + std::fmt::Debug>(node: &mut TreeNode<T>, borrowed_node: Option<&mut TreeNode<T>>) {
            let rc_parent = node.parent.take().unwrap();
            //Handles situation where parent is already borrowed so upper node is passed from outside
            if let Some(upper_node) = borrowed_node {
                if Rc::ptr_eq(&upper_node.root.clone().unwrap(), &rc_parent) {
                    //node is parent's left child
                    if node.key < upper_node.key {
                        upper_node.left_child = None;
                    //node is parent's right child
                    } else if node.key > upper_node.key {
                        upper_node.right_child = None;
                    }
                    return
                }
            }
            let mut parent = rc_parent.borrow_mut();
            //node is parent's left child
            if node.key < parent.key {
                parent.left_child = None;
            //node is parent's right child
            } else if node.key > parent.key {
                parent.right_child = None;
            }
        }
        // Case 3 node with one child and maybe a parent
        fn handle_case_3<T: PartialOrd>(node: &mut TreeNode<T>, borrowed_node: Option<&mut TreeNode<T>>) -> Option<Rc<RefCell<TreeNode<T>>>>{
            let rc_child = node.left_child.take().unwrap_or_else(|| node.right_child.take().unwrap());
            let mut child  = rc_child.borrow_mut();
            
            //Handles situation where parent is already borrowed so upper node is passed from outside
            if let Some(upper_node) = borrowed_node {
                let rc_parent = node.parent.take().unwrap();
                if Rc::ptr_eq(&upper_node.root.clone().unwrap(), &rc_parent) {
                    //node is parent's left child
                    if node.key < upper_node.key {
                        upper_node.left_child = child.root.clone();
                        child.parent = upper_node.root.clone();
                    //node is parent's right child
                    } else if node.key > upper_node.key {
                        upper_node.right_child = child.root.clone();
                        child.parent = upper_node.root.clone();
                    }
                    node.parent = None;
            

                    return None
                }
            }

                //parent exists
                if let Some(rc_parent) = node.parent.take() {
                    let mut parent = rc_parent.borrow_mut();
                    //node is parent's left child
                    if node.key < parent.key {
                        parent.left_child = child.root.clone();
                        child.parent = parent.root.clone();
                    //node is parent's right child
                    } else if node.key > parent.key {
                        parent.right_child = child.root.clone();
                        child.parent = parent.root.clone();
                    }
                    node.parent = None;
                    return None
                //parent doesn't exist 
                } else {
                    child.parent = None;
                    return child.root.clone()
                }
        }
        // Case 4 node with two children and maybe a parent
        fn handle_case_4<T: PartialOrd + std::fmt::Debug>(mut node: &mut TreeNode<T>) -> Option<Rc<RefCell<TreeNode<T>>>> {
            fn find_next_node<T>(node: &TreeNode<T>) -> Rc<RefCell<TreeNode<T>>>{
                if let Some(ref rc_l_child) = node.left_child.clone() {
                    return find_next_node(&rc_l_child.borrow())
                } else {
                    return node.root.clone().unwrap()
                }
            }
            //Find in order successor
            let rc_successor = find_next_node(&node.right_child.clone().unwrap().borrow());
            
            // Disjoin the successor node (creates simpler case 2 or 3)
            let mut successor = rc_successor.borrow_mut();
            // Case 2 leaf node with parent
            if successor.left_child.is_none() && successor.right_child.is_none() {
                handle_case_2(&mut successor, Some(&mut node));
            // Case 3 node with one child and maybe a parent (parent guaranteed here)
            } else if successor.left_child.is_none() || successor.right_child.is_none() {
                handle_case_3(&mut successor, Some(&mut node));
            }

            // Use disjoint successor node to replace target delete node in tree
            let mut new_root:Option<Rc<RefCell<TreeNode<T>>>> = None;
            if node.parent.is_none() {
                new_root = successor.root.clone();
            };
            successor.parent = node.parent.take();
            successor.left_child = node.left_child.take();
            successor.right_child = node.right_child.take();

            new_root
        }
        
        let node_to_delete = self.find(key);
        // Ensure tree is not empty (Case 0)
        if let Some(ref rc_node) = node_to_delete {
            let mut node = rc_node.borrow_mut();
            // Case 1 only one node in tree
            if node.left_child.is_none() && node.right_child.is_none() && node.parent.is_none(){
                self.root = None;
            // Case 2 leaf node with parent
            } else if node.left_child.is_none() && node.right_child.is_none() {
                handle_case_2(&mut node, None);
            // Case 3 node with one child and maybe a parent
            } else if node.left_child.is_none() || node.right_child.is_none() {
                self.set_root_after_rotate(handle_case_3(&mut node, None));
            // Case 4 node with two children and maybe a parent
            } else if node.left_child.is_some() && node.right_child.is_some() {
                self.set_root_after_rotate(handle_case_4(&mut node));
            }
            node.root = None; //This action will reduce the reference count to 0 for the deleted node and trigger the object to be deleted
        };
        
        // TODO Rebalance the tree and fix heights

    }
    pub fn find(&self, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if let Some(ref root) = self.root {
            return root.borrow().binary_tree_find(key)
        } else {
            return None
        }
    }
    pub fn insert(&mut self, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        //FIXME Remove this return value (for debugging)
        let new_node = if let Some(root) = &self.root {
            let new_node = root.borrow_mut().binary_tree_insert(key.clone());
            new_node
        } else {
            let rc_node = TreeNode::new(Node::Avl(AvlNode {}), key.clone());
            self.root = Some(rc_node.clone());
            Some(rc_node)
        };

        //Re-Balance the tree starting at new node and then going up to all ancestors
        let mut current_node = new_node.clone();
        while let Some(ref rc_node) = current_node.clone() {
            let balance_factor: i64 = get_balance_factor(&rc_node.borrow()); //.clone()?
                                                                             //println!("{}:{}:{}", balance_factor, node.key, key);
            if balance_factor > 1 && key < rc_node.borrow().left_child.clone().unwrap().borrow().key
            {
                self.set_root_after_rotate(rc_node.borrow_mut().right_rotate());
            } else if balance_factor < -1
                && key > rc_node.borrow().right_child.clone().unwrap().borrow().key
            {
                //println!("1");
                self.set_root_after_rotate(rc_node.borrow_mut().left_rotate());
            } else if balance_factor > 1
                && key > rc_node.borrow().left_child.clone().unwrap().borrow().key
            {
                let rc_left_child = rc_node.borrow().left_child.clone().unwrap();
                self.set_root_after_rotate(rc_left_child.borrow_mut().left_rotate());
                self.set_root_after_rotate(rc_node.borrow_mut().right_rotate());
            } else if balance_factor < -1
                && key < rc_node.borrow().right_child.clone().unwrap().borrow().key
            {
                let rc_right_child = rc_node.borrow().right_child.clone().unwrap();
                self.set_root_after_rotate(rc_right_child.borrow_mut().right_rotate());
                self.set_root_after_rotate(rc_node.borrow_mut().left_rotate());
            } else {
                rc_node.borrow_mut().fix_height();
            }

            //update current node
            current_node = rc_node.borrow().parent.clone();
        }

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
