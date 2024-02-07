use data_structures::AvlTree;

mod data_structures;
mod graphs;
mod sorting;

fn main() {
    let mut avl: AvlTree<i32> = AvlTree::new();
    avl.insert(17);
    avl.insert(9);
    avl.insert(25);
    avl.insert(5);
    avl.insert(12);
    avl.insert(20);
    //avl.insert(19);

    avl.print();
}
