mod avl_tree;
mod red_black_tree;
mod tree;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbt = avl_tree::AvlTree::new();

    rbt.insert(3);
    rbt.insert(2);
    rbt.insert(1);
    rbt.insert(4);
    rbt.insert(5);

    
    rbt.print_inorder();
    println!("Height of RBT tree is {}", rbt.height());
    rbt.print_tree();
}
