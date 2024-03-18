mod avl_tree;
mod tree;
mod red_black_tree;
use std::env;

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    let mut rbt = red_black_tree::RedBlackTree::new();
    rbt.insert(1);
    let n3 = rbt.insert(3);
    rbt.insert(2);
    rbt.insert(4);
    rbt.insert(6);
    rbt.insert(5);
    n3.unwrap().borrow_mut().left_rotate();
    // insert_node(& x, 45);
    // insert_node(& x, 35);
    // insert_node(& x, 75);
    // insert_node(& x, 1);
    // insert_node(& x, 0);
    // insert_node(& x, 90);

    rbt.print_inorder();
    // println!("{:#?}", x.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().key);
}