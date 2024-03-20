mod avl_tree;
mod red_black_tree;
mod tree;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut avl = avl_tree::AvlTree::new();
    let mut rbt = red_black_tree::RedBlackTree::new();

    rbt.insert(3);
    rbt.insert(2);
    rbt.insert(1);
    rbt.insert(4);
    rbt.insert(5);

    avl.insert(3);
    avl.insert(2);
    avl.insert(1);
    avl.insert(4);
    avl.insert(5);

    avl.print_inorder();
    println!("Height of AVL tree is {}", avl.height());
    avl.print_tree();

    rbt.print_inorder();
    println!("Height of Red Black tree is {}", rbt.height());
    rbt.print_tree();
}
