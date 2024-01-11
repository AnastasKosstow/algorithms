use std::collections::{ BinaryHeap, HashMap };

use crate::graphs::graph::{ Graph, Edge, Node, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn bellman_ford_shortest_path(&self, from_node: &Node<N>, to_node: &Node<N>) -> Option<usize> {

        let mut distances: HashMap<&NodeIndex, usize> = self.nodes
            .iter()
            .map(|node| (&node.index, usize::MAX))
            .collect();

        distances.insert(&from_node.index, 0);

        Some(1)
    }
}