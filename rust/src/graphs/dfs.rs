#![allow(dead_code)]

/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    DFS traversal sequence: A -> C -> D -> B
*/

use std::collections::HashSet;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn are_nodes_connected(&self, root_node: &Node<N>) {

        let mut stack: Vec<&NodeIndex> = vec![];
        stack.push(&root_node.index);

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
                if !visited.contains(&edge.node_index) {
                    stack.push(&edge.node_index);
                }
            }
        }
    }
}
