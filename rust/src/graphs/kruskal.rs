use crate::data_structures::DisjointSet;
use crate::graphs::graph::{ Graph, Edge, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn kruskal_mst(&self) -> Vec<(&NodeIndex, &Edge)> {
        if self.nodes.len() == 0 {
            return vec![];
        }
        
        let mut edges = self.edges
            .iter()
            .flat_map(|node_edge| {
                node_edge.1.iter().map(move |edge| (node_edge.0, edge))
            })
            .collect::<Vec<_>>();

        edges.sort_by_key(|(_, edge)| edge.cost);

        let mut union_find = DisjointSet::new(self.nodes.len());
        let mut mst = Vec::new();

        for (node_ix, edge) in edges {
            let x_root = union_find.find(node_ix.ix);
            let y_root = union_find.find(edge.node_index.ix);

            if x_root != y_root {
                mst.push((node_ix, edge));
                union_find.union(x_root, y_root);
            }
        }
        mst
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::graph::{ Graph, GraphType };

    #[test]
    fn kruskal_empty_graph() {
        let graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let mst = graph.kruskal_mst();
        assert!(mst.is_empty());
    }

    #[test]
    fn kruskal_single_node() {
        let mut graph = Graph::new(GraphType::Undirected);
        graph.add_node(1);
        let mst = graph.kruskal_mst();
        assert!(mst.is_empty());
    }

    #[test]
    fn kruskal_disconnected_graph() {
        let mut graph = Graph::new(GraphType::Undirected);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n1, n2, 1);
        graph.add_node(3); // Disconnected node

        let mst = graph.kruskal_mst();
        assert_eq!(mst.len(), 1);
    }
}
