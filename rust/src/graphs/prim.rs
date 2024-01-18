use std::cmp::Reverse;
use std::collections::{ BinaryHeap, HashSet };

use crate::data_structures::DisjointSet;
use crate::graphs::graph::{ Graph, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn prim_mst<'a>(&'a self, root: &'a NodeIndex) -> Vec<(&NodeIndex, isize)> {

        let mut heap: BinaryHeap<Reverse<(isize, &NodeIndex)>> = BinaryHeap::new(); 
        let mut nodes: Vec<&NodeIndex> = Vec::new();
        nodes.push(root);

        let mut mst: Vec<(&NodeIndex, isize)> = Vec::new();
        mst.push((root, 0));
        
        let mut union_find = DisjointSet::new(self.nodes.len());
        let mut visited: HashSet<&NodeIndex> = HashSet::new();
        
        while let Some(node_index) = nodes.pop() {
            visited.insert(root);

            let edges = match self.edges.get(node_index) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if !visited.contains(&edge.node_index) {
                    heap.push(Reverse((edge.cost, &edge.node_index)));
                }
            }

            while let Some(Reverse((cost, next_node_index))) = heap.pop() {
                let x_index = union_find.find(node_index.ix);
                let y_index = union_find.find(next_node_index.ix);
            
                if x_index != y_index {
                    union_find.union(x_index, y_index);
                    nodes.push(next_node_index);
                    mst.push((next_node_index, cost));
                    break;
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
        let n1 = graph.add_node(1);
        let mst = graph.prim_mst(&n1);
        assert_eq!(mst.len(), 1);
    }

    #[test]
    fn prim_disconnected_graph() {
        let mut graph = Graph::new(GraphType::Undirected);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1);

        graph.add_node(3); // Disconnected node

        let mst = graph.prim_mst(&n1);
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

        let mst = graph.prim_mst(&n1);
        let total_cost: isize = mst.iter()
            .map(|x| x.1)
            .sum();

        assert_eq!(total_cost, 9); 
    }
}