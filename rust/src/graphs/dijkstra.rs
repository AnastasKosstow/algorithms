#![allow(dead_code)]

use std::collections::{ BinaryHeap, HashMap };
use std::cmp::Reverse;

use crate::graphs::graph::{ Graph, Edge, Node, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn shortest_path(&self, from_node: &Node<N>, to_node: &Node<N>) -> Option<usize> {
        
        let mut distances: HashMap<&NodeIndex, usize> = self.nodes.iter()
            .map(|node| {
                return if node == from_node { (&node.index, 0) } else { (&node.index, usize::MAX) };
            })
            .collect();

        let mut visited: Vec<&NodeIndex> = vec![];
        let mut heap: BinaryHeap<Reverse<(usize, &NodeIndex)>> = BinaryHeap::new();
        heap.push(Reverse((0, &from_node.index)));

        while let Some(Reverse((cost, node_ix))) = heap.pop() {
            if &cost > distances.get(&node_ix).unwrap_or(&usize::MAX) {
                continue;
            }

            let edges: &Vec<Edge> = match self.edges.get(&node_ix) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if cost + edge.cost < *distances.get(&edge.node_index).unwrap_or(&usize::MAX) {

                    distances.insert(&edge.node_index, cost + edge.cost);
                    if !visited.contains(&&edge.node_index) {
                        heap.push(Reverse((cost + edge.cost, &edge.node_index)));
                    }
                }
            }
            visited.push(&node_ix);
        }

        distances.get(&to_node.index)
            .copied()
            .filter(|&distance| distance != usize::MAX)
        
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::GraphType;

    use super::*;

    #[test]
    fn test_shortest_path_simple() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n3, 1);
        
        let node1 = graph.get_node(n1).unwrap();
        let node3 = graph.get_node(n3).unwrap();

        assert_eq!(graph.shortest_path(node1, node3), Some(2));
    }

    #[test]
    fn test_no_path_exists() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        let node1 = graph.get_node(n1).unwrap();
        let node2 = graph.get_node(n2).unwrap();

        assert_eq!(graph.shortest_path(node1, node2), None);
    }

    #[test]
    fn test_path_to_self() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node(1);

        let node1 = graph.get_node(n1).unwrap();

        assert_eq!(graph.shortest_path(node1, node1), Some(0));
    }
}
