#![allow(dead_code)]

use std::collections::HashSet;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn are_nodes_connected(&self, from_node: &Node<N>, to_node: &Node<N>) -> bool {

        let mut stack: Vec<&NodeIndex> = vec![];
        stack.push(&from_node.index);

        let mut visited: HashSet<&NodeIndex> = HashSet::new();

        while let Some(node_index) = stack.pop() {
            if visited.contains(node_index) {
                continue;
            }
            visited.insert(node_index);

            let edges = match self.edges.get(node_index) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if edge.node_index == to_node.index {
                    return true;
                }

                if !visited.contains(&edge.node_index) {
                    stack.push(&edge.node_index);
                }
            }
        }
        false
    }
}
