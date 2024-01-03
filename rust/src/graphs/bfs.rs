#![allow(dead_code)]

/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    BFS traversal sequence: A -> B -> C -> D
*/

use std::collections::VecDeque;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn bfs(&self, root_node: &Node<N>) {
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        queue.push_back(root_node.index);

        let mut visited: Vec<NodeIndex> = Vec::new();

        while let Some(node_index) = queue.pop_front() {
            if visited.contains(&node_index) {
                continue;
            }

            let edges = match self.edges.get(&node_index) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if !visited.contains(&edge.node_index) && !queue.contains(&edge.node_index) {
                    queue.push_back(edge.node_index);
                }
            }
            visited.push(node_index);
        }
    }
}
