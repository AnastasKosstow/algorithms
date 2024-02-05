use data_structures::AvlTree;

mod data_structures;
mod graphs;
mod sorting;

fn main() {
    let mut avl: AvlTree<i32> = AvlTree::new();
    avl.insert(5);
    avl.insert(3);
    avl.insert(4);
}
