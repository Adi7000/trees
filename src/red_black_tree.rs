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
    pub color: NodeColor,
    pub key: u32,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}


pub fn new(key: u32) -> RedBlackTree{
    let bushra:TreeNode = TreeNode{
        color: NodeColor::Red,
        key: key,
        parent: None,
        left: None,
        right: None,
    };
    Some(Rc::new(RefCell::new(bushra)))
}

pub fn insert_node(rb_tree: &mut RedBlackTree, key: u32) {
    let rcnode = rb_tree.as_ref().unwrap();
    let mut current_node = rcnode.borrow_mut();
    
    //prevent duplicate insertion
    if current_node.key == key {
        return;
    }

    let temp = if key < current_node.key {
        &mut current_node.left
    } else {
        &mut current_node.right
    };

    match temp {
        Some(_) => {
            insert_node(temp, key);
        }
        None => {

            let newnode:TreeNode = TreeNode{
                color: NodeColor::Red,
                key: key,
                parent: Some(Rc::clone(rcnode)),
                left: None,
                right: None,
            }; 

            let newnode = Rc::new(RefCell::new(newnode));
            if key < current_node.key {
                current_node.left = Some(newnode.clone());
                // recolor(&mut current_node.left);
            } else {
                current_node.right = Some(newnode.clone());
                // recolor(&mut current_node.right);
            };
        }
    }
}


pub fn recolor(rb_tree: &mut RedBlackTree){
    let rcnode = rb_tree.as_ref().unwrap();
    let mut current_node = rcnode.borrow_mut();

    if current_node.parent.is_none(){
        // IF THIS IS ROOT, JUST MAKE IT BLACK
        current_node.color = NodeColor::Black;
    }
    else{
            let mut parent_node = current_node.parent.as_ref().unwrap().borrow_mut();
            if let NodeColor::Black = parent_node.color {
                // IF PARENT IS BLACK IS RETURN
                return;
            }

            else{
                //extract grandparent_node
                let grandparent_rcnode = parent_node.parent.as_ref().unwrap();
                let mut grandparent_node = grandparent_rcnode.borrow_mut();

                let mut is_me_left = false;
                if current_node.key < parent_node.key{
                    is_me_left = true;
                }
                
                let mut is_parent_left = false;

                // EXTRACT UNCLE
                let mut uncle_node = if grandparent_node.key < parent_node.key {
                    // UNCLE IS LEFT CHILD
                    grandparent_node.left.as_ref().unwrap().borrow_mut()
                    
                } else {
                     // UNCLE IS RIGHT CHILD
                    is_parent_left = true;
                    grandparent_node.right.as_ref().unwrap().borrow_mut()
                };

               
                match uncle_node.color{
                    NodeColor::Black => {
                        //do rotation
                    }
            
                    NodeColor::Red => {
                        //change uncle and parent to black
                        uncle_node.color = NodeColor::Black;
                        parent_node.color = NodeColor::Black;

                        //change grandparent to red
                        grandparent_node.color = NodeColor::Red;

                        //call recolor on grandparent
                        recolor(&mut parent_node.parent);
                    }
                }
            }
        
    }
}
