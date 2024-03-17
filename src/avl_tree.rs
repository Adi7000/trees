use crate::tree::*;

pub struct AvlNode {
}

pub struct AvlTree<T> {
    root: TreeNode<T>
}
impl<T> AvlTree<T> {
    pub fn insert(self, data: T) {
        self.root.binary_tree_insert(data);
    }
}
