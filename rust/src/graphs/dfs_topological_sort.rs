#![allow(dead_code)]

use std::collections::{ VecDeque, HashSet };

use crate::graphs::graph::{ Graph, Node, NodeIndex, GraphType };

impl<N: Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn dfs_topological_sort(&self) -> Option<Vec<&Node<N>>> {
        if self.ty == GraphType::Undirected {
            return None; // Topological sort not possible for undirected graphs
        }

        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();
        let mut stack = VecDeque::new();

        for node in &self.nodes {
            if !visited.contains(&node.index) {
                if self.dfs_visit(&node.index, &mut visited, &mut temp_mark, &mut stack).is_none() {
                    return None; // Found a cycle, not a DAG
                }
            }
        }

        while let Some(node_index) = stack.pop_front() {
            match self.get_node(node_index) {
                Some(node) => result.push(node),
                None => continue
            }
        }

        Some(result)
    }

    fn dfs_visit(&self, node_index: &NodeIndex, visited: &mut HashSet<NodeIndex>, temp_mark: &mut HashSet<NodeIndex>, stack: &mut VecDeque<NodeIndex>) -> Option<()> {
        if temp_mark.contains(node_index) {
            return None; // Found a cycle
        }

        if !visited.contains(node_index) {
            temp_mark.insert(*node_index);
            if let Some(edges) = self.edges.get(node_index) {
                for edge in edges {
                    self.dfs_visit(&edge.node_index, visited, temp_mark, stack)?;
                }
            }
            visited.insert(*node_index);
            temp_mark.remove(node_index);
            stack.push_front(*node_index);
        }

        Some(())
    }
}
