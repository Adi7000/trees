mod avl_tree;
mod red_black_tree;
mod tree;
use std::{env, io};

fn run_example() {
    let mut avl = avl_tree::AvlTree::new();
    println!("---Inserting 1-14 into AVL tree---");
    for i in 1..15 {
        avl.insert(i);
    }
    avl.print_tree();
    println!("---Inorder traversal---");
    avl.print_inorder();
    println!("---Deleting 3, 4, 2 from AVL tree---");
    avl.delete(3);
    avl.delete(4);
    avl.delete(2);
    avl.print_tree();

    let mut rb = red_black_tree::RedBlackTree::new();
    println!("---Inserting 1-9 into Red-Black tree---");
    for i in 1..10 {
        rb.insert(i);
    }
    rb.print_tree();
    println!("---Inorder traversal---");
    rb.print_inorder();
    println!("---Deleting 3, 4, 2 from Red-Black tree---");
    rb.delete(3);
    rb.delete(4);
    rb.delete(2);
    rb.print_tree();
}

fn run_manual() {
    // user input to choose tree type
    loop {
        println!("Enter 'avl' to use AVL tree or 'rb' to use Red-Black tree");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "avl" {
            run_avl();
            break;
        } else if input == "rb" {
            run_rb();
            break;
        } else {
            println!("Invalid input");
        }
    }
}

fn run_avl() {
    let mut avl = avl_tree::AvlTree::new();
    loop {
        println!("Enter 'i' to insert, 'd' to delete, 'p' to print tree, 'e' to check if empty, 'o' for in-order traversal, 'l' for leaf count, 'h' for height, 'q' to quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "i" {
            println!("Enter number to insert");
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            let num: i32 = num.trim().parse().unwrap();
            avl.insert(num);
        } else if input == "d" {
            if avl.is_empty() {
                println!("Tree is empty");
                continue;
            }
            println!("Enter number to delete");
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            let num: i32 = num.trim().parse().unwrap();
            avl.delete(num);
        } else if input == "p" {
            avl.print_tree();
        } else if input == "q" {
            break;
        } else if input == "e" {
            println!("Tree is empty: {}", avl.is_empty());
        } else if input == "o" {
            avl.print_inorder();
        } else if input == "l" {
            println!("Leaf count: {}", avl.get_number_of_leaves());
        } else if input == "h" {
            println!("Height: {}", avl.height());
        } else {
            println!("Invalid input");
        }
    }
}

fn run_rb() {
    let mut rb = red_black_tree::RedBlackTree::new();
    loop {
        println!("Enter 'i' to insert, 'd' to delete, 'p' to print tree, 'e' to check if empty, 'o' for in-order traversal, 'l' for leaf count, 'h' for height, 'q' to quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "i" {
            println!("Enter number to insert");
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            let num: i32 = num.trim().parse().unwrap();
            rb.insert(num);
        } else if input == "d" {
            if rb.is_empty() {
                println!("Tree is empty");
                continue;
            }
            println!("Enter number to delete");
            let mut num = String::new();
            io::stdin().read_line(&mut num).unwrap();
            let num: i32 = num.trim().parse().unwrap();
            rb.delete(num);
        } else if input == "p" {
            rb.print_tree();
        } else if input == "q" {
            break;
        } else if input == "e" {
            println!("Tree is empty: {}", rb.is_empty());
        } else if input == "o" {
            rb.print_inorder();
        } else if input == "l" {
            println!("Leaf count: {}", rb.get_number_of_leaves());
        } else if input == "h" {
            println!("Height: {}", rb.height());
        } else {
            println!("Invalid input");
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // take user input to run example or loop through user input
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: `cargo run example` or `cargo run manual`");
        return;
    }
    if args[1] == "example" {
        run_example();
    } else if args[1] == "manual" {
        run_manual();
    } else {
        println!("Usage: `cargo run example` or `cargo run manual`");
    }
}
