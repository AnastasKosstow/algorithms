use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

use crate::graphs::graph::{ Graph, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn prim_mst(&self) -> Vec<(&NodeIndex, isize)> {
        if self.nodes.len() == 0 {
            return vec![];
        }

        let mut heap = BinaryHeap::new(); 
        let mut visited = HashSet::new();
        let mut mst = Vec::new();

        let root = self.nodes.first().unwrap();
        heap.push(Reverse((0, &root.index)));

        while let Some(Reverse((cost, node_ix))) = heap.pop() {
            if visited.contains(node_ix) {
                continue;
            }

            visited.insert(node_ix);
            mst.push((node_ix, cost));

            if let Some(edges) = self.edges.get(node_ix) {
                for edge in edges {
                    let next = &edge.node_index;
                    if !visited.contains(next) {
                        heap.push(Reverse((edge.cost, next)));
                    }
                }
            }
        }
        mst
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::GraphType;

    use super::*;

    #[test]
    fn prim_single_node() {
        let mut graph = Graph::new(GraphType::Undirected);
        graph.add_node(1);
        let mst = graph.prim_mst();
        assert_eq!(mst.len(), 1);
    }

    #[test]
    fn prim_disconnected_graph() {
        let mut graph = Graph::new(GraphType::Undirected);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1);

        graph.add_node(3); // Disconnected node

        let mst = graph.prim_mst();
        assert_eq!(mst.len(), 2); 
    }

    #[test]
    fn prim_single_mst() {
        let mut graph = Graph::new(GraphType::Undirected);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n1, n3, 3);
        graph.add_edge(n2, n4, 10);
        graph.add_edge(n3, n4, 5);

        let mst = graph.prim_mst();
        let total_cost: isize = mst.iter()
            .map(|x| x.1)
            .sum();

        assert_eq!(total_cost, 9); 
    }
}