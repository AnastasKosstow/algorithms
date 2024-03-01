use data_structures::RedBlackTree;

mod data_structures;
mod graphs;
mod primes;
mod sorting;

fn main() {
    let mut rbt: RedBlackTree<i32> = RedBlackTree::new();
    rbt.insert(10);
    rbt.insert(15);
    rbt.insert(5);
    rbt.insert(20);
    rbt.insert(17);

    rbt.insert(1114);

}
