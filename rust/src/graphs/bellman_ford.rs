use std::collections::HashMap;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn bellman_ford_shortest_path(&self, from_node: &Node<N>, to_node: &Node<N>) -> Option<isize> {

        let mut distances: HashMap<NodeIndex, isize> = self.nodes
            .iter()
            .map(|node| (node.index, isize::MAX))
            .collect();

        distances.insert(from_node.index, 0);

        let mut updated = true;
        for _ in 0..self.nodes.len() - 1 {
            if !updated {
                break;
            }
            updated = false;

            for node_edges in self.edges.iter() {
                let node_index = node_edges.0;
                let cost = *distances.get(node_index).unwrap_or(&isize::MAX);
    
                for edge in node_edges.1.iter() {
                    if cost + edge.cost < *distances.get(&edge.node_index).unwrap_or(&isize::MAX) {
                        distances.insert(edge.node_index, cost + edge.cost);
                        updated = true;
                    }
                }
            }
        }

        for node_edges in self.edges.iter() {
            let cost = *distances.get(node_edges.0).unwrap_or(&isize::MAX);

            for edge in node_edges.1.iter() {
                if cost + edge.cost < *distances.get(&edge.node_index).unwrap_or(&isize::MAX) {
                    return None;
                }
            }
        }

        distances.get(&to_node.index)
            .copied()
            .filter(|&distance| distance != isize::MAX)
    }
}

