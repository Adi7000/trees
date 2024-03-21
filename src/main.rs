mod avl_tree;
mod red_black_tree;
mod tree;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut avl = avl_tree::AvlTree::new();
    // let mut rbt = red_black_tree::RedBlackTree::new();

    // rbt.insert(3);
    // rbt.insert(2);
    // rbt.insert(1);
    // rbt.insert(4);
    // rbt.insert(5);

    avl.insert(1);
    let n2 = avl.insert(2);
    avl.insert(3);
    let n4 = avl.insert(4);
    avl.insert(5);
    avl.print_inorder();
    avl.print_tree();
    avl.delete(3);
    avl.delete(4);
    avl.delete(2);

    avl.print_inorder();
    avl.print_tree();

    //println!("{}", n2.unwrap().borrow().right_child.clone().unwrap().borrow().key)

    // rbt.print_inorder();
    // println!("Height of Red Black tree is {}", rbt.height());
    // rbt.print_tree();
}
