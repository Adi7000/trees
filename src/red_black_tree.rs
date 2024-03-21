use std::{cell::RefCell, rc::Rc};

use crate::tree;
use crate::tree::TreeNode;
use crate::tree::Node;

pub struct RedBlackTree<T> {
    pub root: Option<Rc<RefCell<tree::TreeNode<T>>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct RedBlackNode {
    pub color: NodeColor,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NodeColor {
    Red,
    Black,
}

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }
    pub fn is_empty(&self) -> bool {
        if self.root.is_none() {
            true
        } else {
            false
        }
    }
    pub fn insert(&mut self, key: T) -> Option<Rc<RefCell<tree::TreeNode<T>>>> {
        if let Some(root) = &self.root {
            let new_node = root.borrow_mut().binary_tree_insert(key);
            new_node.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
            self.recolor(&new_node); // Handle recoloring here
            new_node
        } else {
            let red_black_node = RedBlackNode {
                color: NodeColor::Black,
            };
            let rc_node = tree::TreeNode::new(tree::Node::RedBlack(red_black_node), key);
            self.root = Some(rc_node.clone());
            Some(rc_node)
        }
    }


    

    pub fn delete(& mut self, key: T){

        if self.root.is_none(){
            return;
        }

        if self.root.as_ref().unwrap().borrow().binary_tree_find(key.clone()).is_none(){
            return;
        }
        // VARIABLE TO HOLD NODE TO BE DELETED
        let delete_node_rc: Rc<RefCell<TreeNode<T>>> =  Rc::clone(& self.root.as_ref().unwrap().borrow().binary_tree_find(key).as_ref().unwrap());

        let mut delete_node_color: NodeColor = NodeColor::Black;
        if let tree::Node::RedBlack(rbt) = delete_node_rc.borrow().kind {
            delete_node_color = rbt.color.clone();
        }

        let mut x: Option<Rc<RefCell<TreeNode<T>>>>;
        if delete_node_rc.borrow().right_child.is_none() {
            x = delete_node_rc.borrow().left_child.clone();
            self.root = self.transplant(&delete_node_rc.borrow().root, &x);
        }
        else if delete_node_rc.borrow().left_child.clone().is_none() {
            x = delete_node_rc.borrow().right_child.clone();
            self.root = self.transplant(&delete_node_rc.borrow().root, &x);
        }
        else {
            // FIND MINIMUM RIGHT SUB TREE
            let mut in_order_min: Option<Rc<RefCell<TreeNode<T>>>> = delete_node_rc.borrow().right_child.clone();

            while in_order_min.as_ref().unwrap().borrow().left_child.is_some() {
                let temp: Rc<RefCell<TreeNode<T>>> = Rc::clone(&in_order_min.as_ref().unwrap().borrow().left_child.as_ref().unwrap());
                in_order_min = Some(Rc::clone(&temp));
            }

            if let tree::Node::RedBlack(rbt) = in_order_min.as_ref().unwrap().borrow().kind {
                delete_node_color = rbt.color.clone();
            }

            x = in_order_min.as_ref().unwrap().borrow().right_child.clone();

            if in_order_min.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key ==  delete_node_rc.borrow().key {
                if x.is_some(){
                    x.as_ref().unwrap().borrow_mut().parent = in_order_min.clone();
                }
            }
            else {
                self.root = self.transplant(&in_order_min, &in_order_min.as_ref().unwrap().borrow().right_child );
                in_order_min.as_ref().unwrap().borrow_mut().right_child = delete_node_rc.borrow().right_child.clone();
                in_order_min.as_ref().unwrap().borrow().right_child.as_ref().unwrap().borrow_mut().parent = in_order_min.clone();
            }

            self.root = self.transplant(&delete_node_rc.borrow().root, &in_order_min);
            self.root.as_ref().unwrap().borrow_mut().parent = None;

            in_order_min.as_ref().unwrap().borrow_mut().left_child = Some(Rc::clone(&delete_node_rc.borrow().left_child.as_ref().unwrap()));
            in_order_min.as_ref().unwrap().borrow().left_child.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(&in_order_min.as_ref().unwrap()));
            in_order_min.as_ref().unwrap().borrow_mut().kind = delete_node_rc.borrow().kind.clone();
            self.root.as_ref().unwrap().borrow_mut().fix_height();
            in_order_min.as_ref().unwrap().borrow_mut().fix_height();
        }


        match delete_node_color {
            NodeColor::Black => {
                self.fix_delete(& x);
            }
            NodeColor::Red => {
                return;
            }
        }        
    }



    pub fn transplant(
        & self, 
        u: & Option<Rc<RefCell<tree::TreeNode<T>>>>, 
        v: & Option<Rc<RefCell<tree::TreeNode<T>>>>
    ) -> Option<Rc<RefCell<tree::TreeNode<T>>>>{

        let rcnode = Rc::clone(& u.as_ref().unwrap());

        if rcnode.borrow().parent.is_none() {
            return v.clone();
        }
        else {
            let rc_parent: Rc<RefCell<tree::TreeNode<T>>> = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
            if rcnode.borrow().key < rc_parent.borrow().key {
                rc_parent.borrow_mut().left_child = v.clone();
            }
            else {
                rc_parent.borrow_mut().right_child = v.clone();
            }
        }
        if v.is_some(){
            v.as_ref().unwrap().borrow_mut().parent = rcnode.borrow().parent.clone();
        }
       
        return self.root.clone();
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

    fn parent_is_red(&mut self, rc_tree: & Rc<RefCell<tree::TreeNode<T>>>) -> bool {
        let parent_rb_node = &rc_tree.borrow().parent;
        if parent_rb_node.is_none(){
            return false;
        }
        
        if let tree::Node::RedBlack(rbt) = parent_rb_node.as_ref().unwrap().borrow().kind {
            if let NodeColor::Red = rbt.color.clone(){
                return true;
            }
            return false;
        }
        return false;
    }

    fn am_i_black(&mut self, rc_tree: & Rc<RefCell<tree::TreeNode<T>>>) -> bool {
        if let tree::Node::RedBlack(rbt) = rc_tree.borrow().kind {
            if let NodeColor::Black = rbt.color.clone(){
                return true;
            }
            return false;
        }
        return false;
    }

    fn node_is_black(&mut self, rb_tree: & Option<Rc<RefCell<tree::TreeNode<T>>>>) -> bool {

        if rb_tree.is_none(){
            return true;
        }

        if let tree::Node::RedBlack(rbt) = rb_tree.as_ref().unwrap().borrow().kind {
            if let NodeColor::Black = rbt.color.clone(){
                return true;
            }
            return false;
        }
        return false;
    }



    pub fn fix_delete(&mut self, rb_tree: & Option<Rc<RefCell<tree::TreeNode<T>>>>){

        if rb_tree.is_none(){
            return;
        }

        let mut rcnode = Rc::clone(& rb_tree.as_ref().unwrap());

        while self.am_i_black(&rcnode) && rcnode.borrow().parent.is_some(){

            // FINDOUT WHICH CHILD AM I
            let rc_parent: Rc<RefCell<tree::TreeNode<T>>> = Rc::clone(& rcnode.borrow().parent.as_ref().unwrap());
            
            let mut is_me_left = false;

            if rcnode.borrow().key < rc_parent.borrow().parent.as_ref().unwrap().borrow().key{
                is_me_left = true;
            }

            if is_me_left {
                // SIBLING IS RIGHT CHILD
                let mut rc_sibling: Rc<RefCell<tree::TreeNode<T>>> = Rc::clone(& rc_parent.borrow().right_child.as_ref().unwrap());
                
                let mut sibling_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
                if let tree::Node::RedBlack(rbt) = rc_sibling.borrow().kind {
                    sibling_color = rbt.color.clone();
                }

                // CASE 1: SIBLING WAS RED  
                match sibling_color {
                    NodeColor::Red => { 
                        rc_sibling.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                        rc_parent.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Red});
                        let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_parent.borrow_mut().left_rotate();
                        if rotation.is_some() {
                            self.root = rotation;
                        }
                        rc_sibling = Rc::clone(&rc_parent.borrow().right_child.as_ref().unwrap())
                    }
                    NodeColor::Black => {}
                }   

                // CASE 2: BOTH NEPHEWS ARE BLACK
                if self.node_is_black(&rc_sibling.borrow().right_child) && self.node_is_black(&rc_sibling.borrow().left_child) {
                    rc_sibling.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Red});
                    rcnode = Rc::clone(&rc_parent);
                }

                else{
                    // CASE 3: RIGHT NEPHEW IS BLACK
                    if self.node_is_black(&rc_sibling.borrow().right_child) {
                        rc_sibling.borrow().left_child.as_ref().unwrap().borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                        rc_sibling.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

                        let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_sibling.borrow_mut().left_rotate();
                        if rotation.is_some() {
                            self.root = rotation;
                        }
                        rc_sibling = Rc::clone(&rc_parent.borrow().right_child.as_ref().unwrap());
                    }

                    // CASE 4: DEFAULT SORT OF
                    rc_sibling.borrow_mut().kind =  rc_parent.borrow().kind.clone();
                    rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                    rc_sibling.borrow().right_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                    let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_parent.borrow_mut().left_rotate();
                    if rotation.is_some() {
                        self.root = rotation;
                    }
                    
                    rcnode = Rc::clone(&self.root.as_ref().unwrap());
                }

            }
            else {

                // SIBLING IS LEFT CHILD
                let mut rc_sibling: Rc<RefCell<tree::TreeNode<T>>> = Rc::clone(& rc_parent.borrow().left_child.as_ref().unwrap());
                
                let mut sibling_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
                if let tree::Node::RedBlack(rbt) = rc_sibling.borrow().kind {
                    sibling_color = rbt.color.clone();
                }

                // CASE 1: SIBLING WAS RED  
                match sibling_color {
                    NodeColor::Red => { 
                        rc_sibling.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                        rc_parent.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Red});
                        let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_parent.borrow_mut().right_rotate();
                        if rotation.is_some() {
                            self.root = rotation;
                        }
                        rc_sibling = Rc::clone(&rc_parent.borrow().left_child.as_ref().unwrap())
                    }
                    NodeColor::Black => {}
                }   

                // CASE 2: BOTH NEPHEWS ARE BLACK
                if self.node_is_black(&rc_sibling.borrow().right_child) && self.node_is_black(&rc_sibling.borrow().left_child) {
                    rc_sibling.borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Red});
                    rcnode = Rc::clone(&rc_parent);
                }

                else{
                    // CASE 3: RIGHT NEPHEW IS BLACK
                    if self.node_is_black(&rc_sibling.borrow().left_child) {
                        rc_sibling.borrow().right_child.as_ref().unwrap().borrow_mut().kind =  Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                        rc_sibling.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

                        let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_sibling.borrow_mut().left_rotate();
                        if rotation.is_some() {
                            self.root = rotation;
                        }
                        rc_sibling = Rc::clone(&rc_parent.borrow().left_child.as_ref().unwrap());
                    }

                    // CASE 4: DEFAULT SORT OF
                    rc_sibling.borrow_mut().kind =  rc_parent.borrow().kind.clone();
                    rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                    rc_sibling.borrow().left_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                    let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_parent.borrow_mut().right_rotate();
                    if rotation.is_some() {
                        self.root = rotation;
                    }
                    
                    rcnode = Rc::clone(&self.root.as_ref().unwrap());
                }
            }
        }

        rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
        // rcnode.borrow_mut().parent = None;
        // println!("IN FIX DELETE {:#?}", self.root.as_ref().unwrap().borrow().key);
        // self.root = Some(Rc::clone(&rcnode));
    }

    fn recolor(&mut self, rb_tree: & Option<Rc<RefCell<tree::TreeNode<T>>>>){
        // GET AN OWNED IMMUTABLE REFERENCE TO CHILD NODE
        let mut rcnode = Rc::clone(& rb_tree.as_ref().unwrap());

        while self.parent_is_red(&rcnode) {
            // EXTRACT PARENT
            let rc_parent = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());

            if rc_parent.borrow().parent.is_some() {
                // EXTRACT GRAND PARENT
                let rc_grandparent = Rc::clone(&rc_parent.borrow().parent.as_ref().unwrap());

                // WHICH CHILD IS PARENT AND UNCLE
                let mut is_me_left = false;
                let mut is_parent_left = true;
                if rcnode.borrow().key < rc_parent.borrow().key{
                    is_me_left = true;
                }
                if rc_grandparent.borrow().key < rc_parent.borrow().key{
                    is_parent_left = false;
                }

                if is_parent_left{
                    // IF PARENT IS THE LEFT CHILD

                    let mut uncle_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
                    if rc_grandparent.borrow().right_child.as_ref().is_some() {
                        if let tree::Node::RedBlack(rbt) = rc_grandparent.borrow().right_child.as_ref().unwrap().borrow().kind {
                            uncle_color = rbt.color.clone();
                        }
                    }

                    match uncle_color {

                        NodeColor::Black => {       
                            // IF UNCLE IS BLACK

                            if !is_me_left {
                                rcnode = Rc::clone(&rc_parent);
                                let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rcnode.borrow_mut().left_rotate();
                                if rotation.is_some() {
                                    self.root = rotation;
                                }
                            }

                            rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});                            
                            rc_grandparent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
                            let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_grandparent.borrow_mut().right_rotate();
                            if rotation.is_some() {
                                self.root = rotation;
                            }
                        }
                        NodeColor::Red => {
                            rc_grandparent.borrow().right_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                            rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                            rc_grandparent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

                            rcnode.borrow_mut().fix_height();
                            rc_parent.borrow_mut().fix_height();

                            rcnode = Rc::clone(&rc_grandparent);
                            continue;
                        }
                    }
                }
                else {
                    // IF PARENT IS THE RIGHT CHILD
                    let mut uncle_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
                    if rc_grandparent.borrow().left_child.as_ref().is_some() {
                        if let tree::Node::RedBlack(rbt) = rc_grandparent.borrow().left_child.as_ref().unwrap().borrow().kind {
                            uncle_color = rbt.color.clone();
                        }
                    }

                    match uncle_color {
                        NodeColor::Black => {
                            // IF UNCLE IS BLACK

                            if is_me_left {
                                rcnode = Rc::clone(&rc_parent);
                                let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rcnode.borrow_mut().right_rotate();
                                if rotation.is_some() {
                                    self.root = rotation;
                                }
                            }

                            rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                            rc_grandparent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

                            let rotation: Option<Rc<RefCell<TreeNode<T>>>> = rc_grandparent.borrow_mut().left_rotate();
                            if rotation.is_some() {
                                self.root = rotation;
                            }
                        }
                        NodeColor::Red => {
                            rc_grandparent.borrow().left_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                            rc_parent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
                            rc_grandparent.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

                            rcnode.borrow_mut().fix_height();
                            rc_parent.borrow_mut().fix_height();

                            rcnode = Rc::clone(&rc_grandparent);
                            continue;
                        }
                    }
                }
            }
        }

        // THE ITERATOR MAY NOT HAVE REACHED ROOT YET
        // FIX HEIGHT UP TILL ROOT
        rcnode.borrow_mut().fix_height();
        while rcnode.borrow().parent.is_some() {
            rcnode.borrow().parent.as_ref().unwrap().borrow_mut().fix_height();
            let x: Rc<RefCell<TreeNode<T>>> = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
            // println!("{:#?} {:#?}", rcnode.borrow().key, rcnode.borrow().parent.as_ref().unwrap().borrow().key);
            rcnode = Rc::clone(&x);
        }
        self.root.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
    }
}
