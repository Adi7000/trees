mod avl_tree;
mod red_black_tree;
mod tree;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbt = red_black_tree::RedBlackTree::new();

    rbt.insert(3);
    let n4 = rbt.insert(4);
    let n5 = rbt.insert(5);
    let n6 = rbt.insert(6);
    rbt.insert(1);
    rbt.insert(2);

    n4.unwrap().borrow_mut().left_rotate();
    n5.clone().unwrap().borrow_mut().left_rotate();
    n6.unwrap().borrow_mut().right_rotate();
    n5.unwrap().borrow_mut().right_rotate();

    
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
