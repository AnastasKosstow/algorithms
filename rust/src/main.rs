use data_structures::BinarySearchTree;

mod data_structures;
mod graphs;
mod sorting;

fn main() {
    let mut tree = BinarySearchTree::new();

    println!("length: {}", tree.length);

    tree.insert(4);
    tree.insert(3);
    tree.insert(5);
    tree.insert(2);

    println!("length: {}", tree.length);

    tree.delete(5);

    println!("length: {}", tree.length);

    tree.clear();

    println!("length: {}", tree.length);
}
