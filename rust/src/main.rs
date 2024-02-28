use data_structures::RedBlackTree;

mod data_structures;
mod graphs;
mod primes;
mod sorting;

fn main() {
    let mut rbt: RedBlackTree<i32> = RedBlackTree::new();
    rbt.insert(10);
    rbt.insert(15);
    rbt.insert(6);
    rbt.insert(3);
    rbt.insert(7);
    rbt.insert(1);
    rbt.insert(4);
    rbt.insert(2);

    rbt.insert(1114);

}
