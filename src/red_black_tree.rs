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
// type RedBlackTree = Option<Rc<RefCell<TreeNode>>>;

impl<T: Ord + Clone + std::fmt::Debug + std::fmt::Display> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }
    pub fn find(&self, key: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if let Some(ref root) = self.root {
            return root.borrow().binary_tree_find(key)
        } else {
            return None
        }
    }
    pub fn insert(&mut self, key: T) -> Option<Rc<RefCell<tree::TreeNode<T>>>> {
        //FIXME Remove this return value (for debugging)
        if let Some(root) = &self.root {
            let new_node = root.borrow_mut().binary_tree_insert(key);
            new_node.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
            self.recolor(&new_node);
            //TODO Handle recoloring here
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
        
        if let tree::Node::RedBlack(RBT) = parent_rb_node.as_ref().unwrap().borrow().kind {
            if let NodeColor::Red = RBT.color.clone(){
                return true;
            }
            return false;
        }
        return false;
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
                        if let tree::Node::RedBlack(RBT) = rc_grandparent.borrow().right_child.as_ref().unwrap().borrow().kind {
                            uncle_color = RBT.color.clone();
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

                            rc_node.borrow_mut().fix_height();
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
                        if let tree::Node::RedBlack(RBT) = rc_grandparent.borrow().left_child.as_ref().unwrap().borrow().kind {
                            uncle_color = RBT.color.clone();
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

                            rc_node.borrow_mut().fix_height();
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
            rc_node = rcnode.borrow().parent
        }
        self.root.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
    }
}






// fn recolor(&mut self, rb_tree: & Option<Rc<RefCell<tree::TreeNode<T>>>>){
//     // GET AN OWNED IMMUTABLE REFERENCE TO CHILD NODE
//     let rcnode = Rc::clone(& rb_tree.as_ref().unwrap());

//     // CHECK IF ITS THE ROOT
//     let mut parent_is_none = rcnode.borrow().parent.is_none();
//     if parent_is_none == true{
//         // IF THIS IS ROOT, JUST MAKE IT BLACK
//         // println!("{:#?}", rcnode);
//         rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//     }
//     else{
//         // IF CHILD I.E. THE NEWLY INSERTED NODE IS NOT ROOT

//         // GET AN IMMUTABLE REFERENCE TO PARENT
//         let parent_node: Rc<RefCell<TreeNode<T>>> = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
//         let mut parent_color: NodeColor = NodeColor::Black;

//         if let tree::Node::RedBlack(RBT) = parent_node.borrow().kind {
//             parent_color = RBT.color.clone();
//         }


//         println!("{:#?}", parent_color);

//         match parent_color {
//             NodeColor::Black => {
//                 // IF PARENT IS BLACK, DONT NEED REBALANCING
//                 return;
//             }
//             NodeColor::Red => {

//                 // SINCE PARENT IS RED, NEED TO GET UNCLE (FOR THIS NEED GRAND PARENT)
//                 if parent_node.borrow().parent.as_ref().is_some() {

//                     // EXTRACT GRAND PARENT IMMUTABLE REFERENCE
//                     let grandparent_rcnode: Rc<RefCell<TreeNode<T>>> = Rc::clone(&parent_node.borrow().parent.as_ref().unwrap());

//                     // WHICH CHILD AM I (NEEDED IN ROTATION STEP)
//                     let mut is_me_left = false;
//                     let mut is_parent_left = true;
//                     if rcnode.borrow().key < parent_node.borrow().key{
//                         is_me_left = true;
//                     }

//                     // WHICH CHILD IS PARENT (NEEDED IN ROTATION STEP)
//                     if grandparent_rcnode.borrow().key < parent_node.borrow().key{
//                         is_parent_left = false;
//                     }

//                     // GET UNCLE COLOR
//                     let mut uncle_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
//                     if is_parent_left {
//                         if grandparent_rcnode.borrow().right_child.as_ref().is_some() {
//                             if let tree::Node::RedBlack(RBT) = grandparent_rcnode.borrow().right_child.as_ref().unwrap().borrow().kind {
//                                 uncle_color = RBT.color.clone();
//                             }
//                         }
//                     }
//                     else {
//                         if grandparent_rcnode.borrow().left_child.as_ref().is_some() {
//                             if let tree::Node::RedBlack(RBT) = grandparent_rcnode.borrow().left_child.as_ref().unwrap().borrow().kind {
//                                 uncle_color = RBT.color.clone();
//                             }
//                         }
//                     }

//                     // LET THE RECOLORING BEGIN
//                     if let NodeColor::Black = uncle_color {
//                         // IF UNCLE BLACK, DO ROTATION

//                         println!("LET THE ROTATION BEGIN");
//                         if is_me_left && is_parent_left {
//                             /*
//                                 1. HANDLE LL CASE
//                                 - RIGHT ROTATION ON GRAND PARENT
//                                 - SWAP COLOR OF PARENT AND GRAND PARENT
//                             */
//                             parent_node.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                             grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
//                             let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().right_rotate();
//                             if rotation.is_some() {
//                                 self.root = rotation;
//                             }

//                     //         // let parent_color: tree::Node = parent_node.borrow().kind.clone();
//                     //         // let grandparent_color: tree::Node = grandparent_rcnode.borrow().kind.clone();
//                     //         // parent_node.borrow_mut().kind = grandparent_color;
//                     //         // grandparent_rcnode.borrow_mut().kind = parent_color;
//                             // parent_node.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                             // grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
//                     //         let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().right_rotate();
//                     //         if rotation.is_some() {
//                     //             self.root = rotation;
//                     //         }
//                             self.recolor(&rcnode.borrow().root);
//                         }
//                         if !is_me_left && !is_parent_left {
//                             /*
//                                 2. HANDLE RR CASE
//                                 - LEFT ROTATION ON GRAND PARENT
//                                 - SWAP COLOR OF PARENT AND GRAND PARENT
//                             */
//                             parent_node.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                             grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
//                             let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().left_rotate();
//                             if rotation.is_some() {
//                                 self.root = rotation;
//                             }
//                             self.recolor(&rcnode.borrow().root);
//                     //         self.recolor(&rcnode.borrow().root);
//                     //         // let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().left_rotate();
//                     //         // if rotation.is_some() {
//                     //         //     self.root = rotation;
//                     //         // }

//                     //         // let parent_color: tree::Node = parent_node.borrow().kind.clone();
//                     //         // let grandparent_color: tree::Node = grandparent_rcnode.borrow().kind.clone();
//                     //         // parent_node.borrow_mut().kind = grandparent_color;
//                     //         // grandparent_rcnode.borrow_mut().kind = parent_color;
                            
//                         }
//                     //     if !is_me_left && is_parent_left {
//                     //         // 3. HANDLE LR CASE
//                     //         let rotation: Option<Rc<RefCell<TreeNode<T>>>> = parent_node.borrow_mut().left_rotate();
//                     //         if rotation.is_some() {
//                     //             self.root = rotation;
//                     //         }
                            
//                     //         parent_node.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                     //         grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});
//                     //         let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().right_rotate();
//                     //         if rotation.is_some() {
//                     //             self.root = rotation;
//                     //         }
//                     //         self.recolor(&rcnode.borrow().root);


//                     //         // let my_color: tree::Node = rcnode.borrow().kind.clone();
//                     //         // let grandparent_color: tree::Node = grandparent_rcnode.borrow().kind.clone();
//                     //         // rcnode.borrow_mut().kind = grandparent_color;
//                     //         // grandparent_rcnode.borrow_mut().kind = my_color;
                            
//                     //     }
//                     //     if is_me_left && !is_parent_left {
//                     //         /*
//                     //          4. HANDLE RL CASE
//                     //         - RIGHT ROTATE PARENT
//                     //         - SWAP COLOR OF CURRENT AND GRAND PARENT
//                     //         */
//                     //         let rotation: Option<Rc<RefCell<TreeNode<T>>>> = parent_node.borrow_mut().right_rotate();
//                     //         if rotation.is_some() {
//                     //             self.root = rotation;
//                     //         }
                        
//                     //         parent_node.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                     //         grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

//                     //         let rotation: Option<Rc<RefCell<TreeNode<T>>>> = grandparent_rcnode.borrow_mut().left_rotate();
//                     //         if rotation.is_some() {
//                     //             self.root = rotation;
//                     //         }
//                     //         // let my_color: tree::Node = rcnode.borrow().kind.clone();
//                     //         // let grandparent_color: tree::Node = grandparent_rcnode.borrow().kind.clone();
//                     //         // rcnode.borrow_mut().kind = grandparent_color;
//                     //         // grandparent_rcnode.borrow_mut().kind = my_color;
//                     //         self.recolor(&rcnode.borrow().root);
                            
//                     //     }
//                     }
//                     else {
//                         // SINCE UNCLE IS RED, DO THE FOLLOWING

//                         println!("BEFORE RECOLOR");
//                         println!(
//                             "child {:#?}, parent {:#?}, GB {:#?}, uncle {:#?}",
//                             rcnode.borrow().kind,
//                             parent_node.borrow().kind,
//                             grandparent_rcnode.borrow().kind,
//                             grandparent_rcnode.borrow().left_child.as_ref().unwrap().borrow().kind
//                         );

//                         //1.  CHANGE UNCLE TO BLACK
//                         if is_parent_left {
//                             grandparent_rcnode.borrow().right_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                         }
//                         else {
//                             grandparent_rcnode.borrow().left_child.as_ref().unwrap().borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});
//                         }

//                         //2.  CHANGE PARENT TO BLACK
//                         let parentrcnew = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
//                         parentrcnew.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Black});

//                         //3. CHANGE GRAND PARENT TO RED
//                         grandparent_rcnode.borrow_mut().kind = Node::RedBlack(RedBlackNode {color: NodeColor::Red});

//                         println!("AFTER RECOLOR");
//                         println!(
//                             "child {:#?}, parent {:#?}, GB {:#?}, uncle {:#?}",
//                             rcnode.borrow().kind,
//                             parent_node.borrow().kind,
//                             grandparent_rcnode.borrow().kind,
//                             grandparent_rcnode.borrow().left_child.as_ref().unwrap().borrow().kind
//                         );

//                         //4. CALL RECOLOR ON GRAND PARENT
//                         self.recolor(&parent_node.borrow().parent);
//                     }
//                 }
//             }
//         };
//     }
// }



