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
