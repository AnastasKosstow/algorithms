use std::collections::HashMap;

use crate::graphs::graph::{ Graph, Node };

impl<N: Ord + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn bellman_ford_shortest_path(&self, from_node: &Node<N>, to_node: &Node<N>) -> Option<isize> {
        if self.nodes.len() == 0 {
            return Some(0);
        }
        
        let mut distances = self.nodes
            .iter()
            .map(|node| (node.index, isize::MAX))
            .collect::<HashMap<_, _>>();

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


#[cfg(test)]
mod tests {
    use crate::graphs::GraphType;

    use super::*;

    #[test]
    fn find_path() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node("А");
        let n2 = graph.add_node("B");
        let n3 = graph.add_node("C");

        graph.add_edge(n1, n2, 5);
        graph.add_edge(n1, n3, 4);
        graph.add_edge(n2, n3, -2);
        
        let node1 = graph.get_node(n1).unwrap();
        let node3 = graph.get_node(n3).unwrap();

        assert_eq!(graph.bellman_ford_shortest_path(node1, node3), Some(3));
    }

    #[test]
    fn negative_cycle() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n1, n1, -1);

        let node1 = graph.get_node(n1).unwrap();
        let node2 = graph.get_node(n2).unwrap();

        assert_eq!(graph.bellman_ford_shortest_path(node1, node2), None);
    }

    #[test]
    fn test_path_to_self() {
        let mut graph = Graph::new(GraphType::Directed);
        let n1 = graph.add_node(1);

        let node1 = graph.get_node(n1).unwrap();

        assert_eq!(graph.bellman_ford_shortest_path(node1, node1), Some(0));
    }
}
