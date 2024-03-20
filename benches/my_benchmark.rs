use std::env;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trees::avl_tree;
use trees::red_black_tree;
use trees::red_black_tree::RedBlackTree;

fn rbt_insert(n: u32) {
    let mut tree = red_black_tree::RedBlackTree::new();
    for i in 0..n {
        tree.insert(i);
    }
}

fn rbt_search(tree: &mut RedBlackTree<u32>, n: u32) {
    for i in 0..n {
        tree.find(i);
    }
}

fn avl_insert(n: u32) {
    let mut tree = avl_tree::AvlTree::new();
    for i in 0..n {
        tree.insert(i);
    }
}

fn avl_search(tree: &mut avl_tree::AvlTree<u32>, n: u32) {
    for i in 0..n {
        tree.find(i);
    }
}

fn insert_rbtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("RB Tree: Insert");
    let tree_size: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in tree_size.iter() {
        let id = format!("RB Tree: Insert {} elements", size);
        group.bench_with_input(id, size, |b, &size| b.iter(|| rbt_insert(size)));
    }
    group.finish();
}

fn search_rbtree(c: &mut Criterion) {
    let mut group = c.benchmark_group("RB Tree: Search");
    let tree_size: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in tree_size.iter() {
        let mut tree = red_black_tree::RedBlackTree::new();
        for i in 0..*size {
            tree.insert(i);
        }
        let low_val = size / 10;
        let id = format!("RB Tree: Search {} lowest values", low_val);
        group.bench_function(id, |b| b.iter(|| rbt_search(&mut tree, black_box(low_val))));
    }
    group.finish();
}

fn insert_avltree(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVL Tree: Insert");
    let tree_size: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in tree_size.iter() {
        let id = format!("AVL Tree: Insert {} elements", size);
        group.bench_with_input(id, size, |b, &size| b.iter(|| avl_insert(size)));
    }
    group.finish();
}

fn search_avltree(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVL Tree: Search");
    let tree_size: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];
    for size in tree_size.iter() {
        let mut tree = avl_tree::AvlTree::new();
        for i in 0..*size {
            tree.insert(i);
        }
        let low_val = size / 10;
        let id = format!("AVL Tree: Search {} lowest values", low_val);
        group.bench_function(id, |b| b.iter(|| avl_search(&mut tree, black_box(low_val))));
    }
    group.finish();
}

criterion_group!(
    benches,
    insert_rbtree,
    search_rbtree,
    insert_avltree,
    search_avltree
);
criterion_main!(benches);
