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
