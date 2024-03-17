// use crate::tree::*;

// pub struct RedBlackNode {
//     color: String //change to your needs
// }

// pub struct RedBlackTree<T> {
//     root: TreeNode<T>
// }

use crate::tree::*;

#[derive(Debug)]
enum NodeColor {
    Red,
    Black
}

#[derive(Debug)]
pub struct RedBlackNode {
    color: NodeColor
}


impl<T: Ord + Clone> TreeNode<T> {
    pub fn new_red_black(key: T) -> TreeNode<T> {
        TreeNode {
            key: key,
            left_child: None,
            right_child: None,
            parent: None,
            kind: Node::RedBlack(RedBlackNode{color: NodeColor::Red}),
        }
    }

    pub fn red_black_insert_node(&mut self, key: T) {
        self.binary_tree_insert(key)
    }
}
