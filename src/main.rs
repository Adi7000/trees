mod avl_tree;
mod red_black_tree;
mod tree;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbt = avl_tree::AvlTree::new();

    rbt.insert(3);
    rbt.print_tree();
    rbt.insert(2);
    rbt.print_tree();
    rbt.insert(1);
    rbt.print_tree();

    
    rbt.print_inorder();
    println!("Height of RBT tree is {}", rbt.height());
    // insert_node(& x, 45);
    // insert_node(& x, 35);
    // insert_node(& x, 75);
    // insert_node(& x, 1);
    // insert_node(& x, 0);
    // insert_node(& x, 90);

    // println!("{:#?}", x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key);
    rbt.print_tree();
}
