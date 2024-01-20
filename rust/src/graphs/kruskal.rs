use crate::data_structures::DisjointSet;
use crate::graphs::graph::{ Graph, Edge, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn kruskal_mst(&self) -> Vec<(&NodeIndex, &Edge)> {

        let mut edges: Vec<(&NodeIndex, &Edge)> = self.edges
            .iter()
            .flat_map(|node_edge| {
                node_edge.1.iter().map(move |edge| (node_edge.0, edge))
            })
            .collect();

        edges.sort_by_key(|key| key.1.cost);

        let mut union_find = DisjointSet::new(self.nodes.len());
        let mut mst: Vec<(&NodeIndex, &Edge)> = Vec::new();

        for edge in edges {
            let x = self.nodes[edge.0.ix].index.ix;
            let y = edge.1.node_index.ix;

            let x_root = union_find.find(x);
            let y_root = union_find.find(y);

            if x_root != y_root {
                mst.push(edge);
                union_find.union(x_root, y_root);
            }
        }

        mst
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::graph::{ Graph, GraphType };

    fn create_test_graph() -> Graph<i32> {
        let mut graph = Graph::new(GraphType::Undirected);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n1, n2, 1); // edge with cost 1
        graph.add_edge(n2, n3, 2); // edge with cost 2
        graph.add_edge(n3, n4, 3); // edge with cost 3
        graph.add_edge(n1, n4, 4); // edge with cost 4

        graph
    }

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
