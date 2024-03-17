use crate::tree::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

// pub struct RedBlackNode {
//     color: String //change to your needs
// }

// pub struct RedBlackTree<T> {
//     root: TreeNode<T>
// }

// #[derive(Debug)]
// pub struct RedBlackNode {
//     color: NodeColor
// }


// impl<T: Ord + Clone> TreeNode<T> {
// }


use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}
type RedBlackTree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    pub color: RefCell<NodeColor>,
    pub key: u32,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
    
}


pub fn new(key: u32) -> RedBlackTree{
    let bushra:TreeNode = TreeNode{
        color: NodeColor::Red.into(),
        key: key,
        parent: None,
        left: None,
        right: None,
    };
    Some(Rc::new(RefCell::new(bushra)))
}

pub fn insert_node(rb_tree: & RedBlackTree, key: u32) {
    let rcnode = Rc::clone(&rb_tree.as_ref().unwrap());

    //prevent duplicate insertion
    if rcnode.borrow().key == key {
        return;
    }

    // check if child is none
    if key < rcnode.borrow().key {
        let root = Rc::clone(&rb_tree.as_ref().unwrap());
        if rcnode.borrow().left.is_none(){
            let newnode:TreeNode = TreeNode{
                color: NodeColor::Red.into(),
                key: key,
                parent: Some(Rc::clone(&rb_tree.as_ref().unwrap())),
                left: None,
                right: None,
            }; 

            let newnode = Rc::new(RefCell::new(newnode));
            root.borrow_mut().left = Some(newnode.clone());
            recolor(&root.borrow().left);
        }
        
        else{
            insert_node(&root.borrow().left, key);
        }
    } else {
        let root = Rc::clone(&rb_tree.as_ref().unwrap());
        if rcnode.borrow().right.is_none(){
            let newnode:TreeNode = TreeNode{
                color: NodeColor::Red.into(),
                key: key,
                parent: Some(Rc::clone(&rb_tree.as_ref().unwrap())),
                left: None,
                right: None,
            }; 

            let newnode = Rc::new(RefCell::new(newnode));
            root.borrow_mut().right = Some(newnode.clone());
            recolor(&root.borrow().right);
        }
        
        else{
            insert_node(&root.borrow().right, key);
        }
    }
}


pub fn recolor(rb_tree: & RedBlackTree){
    // GET AN OWNED IMMUTABLE REFERENCE TO CHILD NODE
    let rcnode = Rc::clone(& rb_tree.as_ref().unwrap());

    // CHECK IF ITS THE ROOT
    let mut parent_is_none = false;
    {
        if rcnode.borrow().parent.is_none(){
            // IF THIS IS ROOT, JUST MAKE IT BLACK
            parent_is_none = true;
        }
    }

    if parent_is_none == true{
        // IF THIS IS ROOT, JUST MAKE IT BLACK
        *rcnode.borrow().color.borrow_mut() = NodeColor::Black;
        return;
    }
    else{

        // GET AN IMMUTABLE REFERENCE TO PARENT
        let parent_node: Rc<RefCell<TreeNode>> = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
        let parent_color: NodeColor = parent_node.borrow().color.borrow().clone();

        match parent_color {
            NodeColor::Black => {return;}
            NodeColor::Red => {
                // SINCE PARENT IS RED, NEED TO GET GRAND PARENT AND UNCLE
                println!("Fahrin");
                if parent_node.borrow().parent.as_ref().is_some() {
                    
                    // EXTRACT GRAND PARENT
                    let grandparent_rcnode: Rc<RefCell<TreeNode>> = Rc::clone(&parent_node.borrow().parent.as_ref().unwrap());
                    
                    // WHICH CHILD AM I
                    let mut is_me_left = false;
                    let mut is_parent_left = true;
                    if rcnode.borrow().key < parent_node.borrow().key{
                        is_me_left = true;
                    }

                    // WHICH CHILD IS PARENT
                    if grandparent_rcnode.borrow().key < parent_node.borrow().key{
                        is_parent_left = false;
                    }

                    
                    // GET UNCLE COLOR
                    let mut uncle_color: NodeColor;
                    if is_parent_left {
                        if grandparent_rcnode.borrow().right.as_ref().is_some() {
                            uncle_color = grandparent_rcnode.borrow().right.as_ref().unwrap().borrow().color.borrow().clone();
                        }
                        else {
                            return;
                        }
                    }
                    else {
                        if grandparent_rcnode.borrow().left.as_ref().is_some() {
                            uncle_color = grandparent_rcnode.borrow().left.as_ref().unwrap().borrow().color.borrow().clone();
                        }
                        else {
                            return;
                        }
                    }

                    if let NodeColor::Black = uncle_color {
                        // IF UNCLE BLACK, DO ROTATION
                    }
                    else {
                        // CHANGE UNCLE TO RED
                        if is_parent_left {
                            grandparent_rcnode.borrow().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black.into();
                        }
                        else {
                            grandparent_rcnode.borrow().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black.into();
                        }

                        // CHANGE PARENT TO RED
                        let parentrcnew = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
                        *parentrcnew.borrow().color.borrow_mut() = NodeColor::Black;

                        // CHANGE GRAND PARENT TO RED
                        *grandparent_rcnode.borrow().color.borrow_mut() = NodeColor::Red;

                        println!("HEREEE");
                        println!(
                            "child {:#?}, parent {:#?}, GB {:#?}, uncle {:#?}", 
                            rcnode.borrow().color, 
                            parent_node.borrow().color, 
                            grandparent_rcnode.borrow().color, 
                            grandparent_rcnode.borrow().left.as_ref().unwrap().borrow().color
                        );

                        // CALL RECOLOR ON GRAND PARENT
                        recolor(&parent_node.borrow().parent);
                    }
                }
            }
        };
    }
}



fn main(){
    let mut x = new(4);
    insert_node(& x, 50);
    insert_node(& x, 80);
    insert_node(& x, 2);
    insert_node(& x, 60);
    insert_node(& x, 45);
    insert_node(& x, 35);
    insert_node(& x, 75);
    insert_node(& x, 1);

    // (&x.unwrap()).borrow().print("".to_string(), false);

    // println!("{:#?}", x);
    // let nj = x.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow();

    // println!("{:#?}", x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key);
}
