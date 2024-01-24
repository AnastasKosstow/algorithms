use crate::graphs::graph::Graph;

impl<N: Ord + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn floyd_warshall_shortest_path(&self) -> Vec<Vec<isize>> {

        let node_count = self.nodes.len();
        let mut distances = vec![vec![isize::MAX; node_count]; node_count];

        for (source_index, node) in self.nodes.iter().enumerate() {
            distances[source_index][source_index] = 0; // Distance to self is 0

            if let Some(edges) = self.edges.get(&node.index) {
                for edge in edges {
                    distances[source_index][edge.node_index.ix] = edge.cost;
                }
            }
        }

        for intermediate_vertex in 0..node_count {
            for source_vertex in 0..node_count {
                for destination_vertex in 0..node_count {
                    let new_distance = std::cmp::min(
                        distances[source_vertex][destination_vertex],
                        distances[source_vertex][intermediate_vertex].saturating_add(distances[intermediate_vertex][destination_vertex]),
                    );

                    distances[source_vertex][destination_vertex] = new_distance;
                }
            }
        }
        distances
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::GraphType;

    #[test]
    fn empty_graph() {
        let graph: Graph<&str> = Graph::new(GraphType::Undirected);

        let result = graph.floyd_warshall_shortest_path();
        assert_eq!(result, Vec::<Vec<isize>>::new());
    }

    #[test]
    fn single_path() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Directed);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        graph.add_edge(a, b, 1);
        
        let result = graph.floyd_warshall_shortest_path();
        assert_eq!(result, vec![vec![0, 1], vec![isize::MAX, 0]]);
    }

    #[test]
    fn multiple_paths() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Directed);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        graph.add_edge(a, b, 3);
        graph.add_edge(b, c, 2);

        let result = graph.floyd_warshall_shortest_path();
        assert_eq!(result, vec![vec![0, 3, 5], vec![isize::MAX, 0, 2], vec![isize::MAX, isize::MAX, 0]]);
    }
}




