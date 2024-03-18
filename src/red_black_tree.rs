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


pub fn new_root_node(key: u32) -> RedBlackTree{
    let bushra:TreeNode = TreeNode{
        color: NodeColor::Black.into(),
        key: key,
        parent: None,
        left: None,
        right: None,
    };
    Some(Rc::new(RefCell::new(bushra)))
}

// INSERT NODE INTO TREE
fn insert_node(rb_tree: & RedBlackTree, key: u32) {
    
    let rc_current_node = Rc::clone(&rb_tree.as_ref().unwrap());

    // PREVENT DUPLICATE INSERTION
    if rc_current_node.borrow().key == key {
        return;
    }

    
    if key < rc_current_node.borrow().key {
        // WILL INSERT ON LEFT SUBTREE
        
        // IF WE ARE CURRENTLY AT LEAF (I.E. NONE), INSERT
        if rc_current_node.borrow().left.is_none(){
            let newnode:TreeNode = TreeNode{
                color: NodeColor::Red.into(),
                key: key,
                parent: Some(Rc::clone(&rb_tree.as_ref().unwrap())),
                left: None,
                right: None,
            }; 

            let newnode = Rc::new(RefCell::new(newnode));
            rc_current_node.borrow_mut().left = Some(newnode.clone());
            recolor(&rc_current_node.borrow().left); // REBALANCE TREE
        }
        
        else{
            // RECURSIVE STEP
            insert_node(&rc_current_node.borrow().left, key);
        }
    } else {
        // WILL INSERT ON RIGHT SUBTREE
        if rc_current_node.borrow().right.is_none(){
            let newnode:TreeNode = TreeNode{
                color: NodeColor::Red.into(),
                key: key,
                parent: Some(Rc::clone(&rb_tree.as_ref().unwrap())),
                left: None,
                right: None,
            }; 

            let newnode = Rc::new(RefCell::new(newnode));
            rc_current_node.borrow_mut().right = Some(newnode.clone());
            recolor(&rc_current_node.borrow().right); // REBALANCE TREE
        }
        
        else{
            // RECURSIVE STEP
            insert_node(&rc_current_node.borrow().right, key);
        }
    }
}


// RECOLOR TREE STARTING FROM NEWLY INSERTED NODE
fn recolor(rb_tree: & RedBlackTree){
    // GET AN OWNED IMMUTABLE REFERENCE TO CHILD NODE
    let rcnode = Rc::clone(& rb_tree.as_ref().unwrap());

    // CHECK IF ITS THE ROOT
    let mut parent_is_none = rcnode.borrow().parent.is_none();
    if parent_is_none == true{
        // IF THIS IS ROOT, JUST MAKE IT BLACK
        // println!("{:#?}", rcnode);
        *rcnode.borrow().color.borrow_mut() = NodeColor::Black;
        return;
    }
    else{
        // IF CHILD I.E. THE NEWLY INSERTED NODE IS NOT ROOT

        // GET AN IMMUTABLE REFERENCE TO PARENT
        let parent_node: Rc<RefCell<TreeNode>> = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
        let parent_color: NodeColor = parent_node.borrow().color.borrow().clone();

        match parent_color {
            NodeColor::Black => {
                // IF PARENT IS BLACK, DONT NEED REBALANCING
                return;
            }
            NodeColor::Red => {

                // SINCE PARENT IS RED, NEED TO GET UNCLE (FOR THIS NEED GRAND PARENT)
                if parent_node.borrow().parent.as_ref().is_some() {
                    
                    // EXTRACT GRAND PARENT IMMUTABLE REFERENCE
                    let grandparent_rcnode: Rc<RefCell<TreeNode>> = Rc::clone(&parent_node.borrow().parent.as_ref().unwrap());

                    // WHICH CHILD AM I (NEEDED IN ROTATION STEP)
                    let mut is_me_left = false;
                    let mut is_parent_left = true;
                    if rcnode.borrow().key < parent_node.borrow().key{
                        is_me_left = true;
                    }

                    // WHICH CHILD IS PARENT (NEEDED IN ROTATION STEP)
                    if grandparent_rcnode.borrow().key < parent_node.borrow().key{
                        is_parent_left = false;
                    }

                    // GET UNCLE COLOR
                    let mut uncle_color: NodeColor = NodeColor::Black; // IF UNCLE IS NONE, THAT IS BLACK
                    if is_parent_left {
                        if grandparent_rcnode.borrow().right.as_ref().is_some() {
                            uncle_color = grandparent_rcnode.borrow().right.as_ref().unwrap().borrow().color.borrow().clone();
                        }
                    }
                    else {
                        if grandparent_rcnode.borrow().left.as_ref().is_some() {
                            uncle_color = grandparent_rcnode.borrow().left.as_ref().unwrap().borrow().color.borrow().clone();
                        }
                    }

                    // LET THE RECOLORING BEGIN
                    if let NodeColor::Black = uncle_color {
                        // IF UNCLE BLACK, DO ROTATION
                    }
                    else {
                        // SINCE UNCLE IS RED, DO THE FOLLOWING

                        println!("BEFORE RECOLOR");
                        println!(
                            "child {:#?}, parent {:#?}, GB {:#?}, uncle {:#?}", 
                            rcnode.borrow().color, 
                            parent_node.borrow().color, 
                            grandparent_rcnode.borrow().color, 
                            grandparent_rcnode.borrow().left.as_ref().unwrap().borrow().color
                        );

                        //1.  CHANGE UNCLE TO BLACK
                        if is_parent_left {
                            grandparent_rcnode.borrow().right.as_ref().unwrap().borrow_mut().color = NodeColor::Black.into();
                        }
                        else {
                            grandparent_rcnode.borrow().left.as_ref().unwrap().borrow_mut().color = NodeColor::Black.into();
                        }

                        //2.  CHANGE PARENT TO BLACK
                        let parentrcnew = Rc::clone(&rcnode.borrow().parent.as_ref().unwrap());
                        *parentrcnew.borrow().color.borrow_mut() = NodeColor::Black;

                        //3. CHANGE GRAND PARENT TO RED
                        *grandparent_rcnode.borrow().color.borrow_mut() = NodeColor::Red;

                        println!("AFTER RECOLOR");
                        println!(
                            "child {:#?}, parent {:#?}, GB {:#?}, uncle {:#?}", 
                            rcnode.borrow().color, 
                            parent_node.borrow().color, 
                            grandparent_rcnode.borrow().color, 
                            grandparent_rcnode.borrow().left.as_ref().unwrap().borrow().color
                        );

                        //4. CALL RECOLOR ON GRAND PARENT
                        recolor(&parent_node.borrow().parent);
                    }
                }
            }
        };
    }
}


// IN ORDER TRAVERSAL FUNCTION
fn print_inorder (rb_tree: & RedBlackTree) {
    if rb_tree.is_none() { 
        return;
    } 
    
    let current_node = rb_tree.as_ref().unwrap().borrow();
    
    print_inorder(& current_node.left);

    println!("(Key: {:#?}, Color: {:#?})", current_node.key, current_node.color.borrow());

    print_inorder(& current_node.right);

}

fn main(){
    let mut x = new_root_node(1);
    insert_node(& x, 2);
    insert_node(& x, 0);
    insert_node(& x, 3); // VERY BASIC RECOLORING SEEMS TO WORK
    // insert_node(& x, 45);
    // insert_node(& x, 35);
    // insert_node(& x, 75);
    // insert_node(& x, 1);
    // insert_node(& x, 0);
    // insert_node(& x, 90);

    print_inorder(&x);
    // println!("{:#?}", x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key);
}
