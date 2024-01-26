use std::collections::{HashMap, HashSet};

use crate::graphs::graph::{ Graph, Edge, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn kosaraju_scc(&self) -> Vec<Vec<NodeIndex>> {
        if self.nodes.len() == 0 {
            return vec![];
        }

        let mut reverse_edges: HashMap<NodeIndex, Vec<Edge>> = HashMap::new();
        for (from_index, edges) in self.edges.iter() {
            for edge in edges.iter() {
                reverse_edges
                    .entry(edge.node_index)
                    .or_insert(Vec::new())
                    .push(Edge {
                        cost: edge.cost,
                        node_index: *from_index,
                    });
            }
        }

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut components: Vec<Vec<NodeIndex>> = Vec::new();

        for (node_index, _) in reverse_edges.iter() {
            if visited.contains(&node_index) {
                continue;
            }
            let mut component = Vec::new();
            find_component_dfs(&reverse_edges, node_index, &mut visited, &mut component);
            components.push(component);
        }

        fn find_component_dfs(
            graph_edges: &HashMap<NodeIndex, Vec<Edge>>, 
            node_index: &NodeIndex, 
            visited: &mut HashSet<NodeIndex>,
            component: &mut Vec<NodeIndex>)
        {
            if visited.contains(&node_index) {
                return;
            }
    
            visited.insert(*node_index);
            
            if let Some(edges) = graph_edges.get(&node_index) {
                for edge in edges {
                    find_component_dfs(graph_edges, &edge.node_index, visited, component);
                }
            }
            
            component.push(*node_index);
        }

        components
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphs::graph::{Graph, GraphType, Node, NodeIndex, Edge};

    fn create_test_graph(edges: Vec<(usize, usize)>) -> Graph<i32> {
        let mut graph = Graph {
            nodes: vec![],
            edges: HashMap::new(),
            ty: GraphType::Directed,
            length: edges.len(),
        };

        for (from, to) in edges {
            graph.nodes.push(Node {
                value: from as i32,
                index: NodeIndex { ix: from },
            });
            graph.edges.entry(NodeIndex { ix: from })
                .or_insert_with(Vec::new)
                .push(Edge {
                    cost: 1,
                    node_index: NodeIndex { ix: to },
                });
        }

        graph
    }
    
    #[test]
    fn empty_graph() {
        let graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let scc = graph.kosaraju_scc();
        assert!(scc.is_empty());
    }

    #[test]
    fn multiple_sccs() {
        let graph = create_test_graph(vec![(0, 1), (1, 2), (2, 0), (3, 4)]);
        let scc = graph.kosaraju_scc();
        assert_eq!(scc.len(), 2);
    }

    #[test]
    fn single_node_graph() {
        let graph = create_test_graph(vec![(0, 0)]);
        let scc = graph.kosaraju_scc();
        assert_eq!(scc.len(), 1);
    }

    #[test]
    fn disconnected_graph() {
        let graph = create_test_graph(vec![(0, 1), (2, 3)]);
        let scc = graph.kosaraju_scc();
        assert_eq!(scc.len(), 2);
    }

    #[test]
    fn cycle_graph() {
        let graph = create_test_graph(vec![(0, 1), (1, 2), (2, 0)]);
        let scc = graph.kosaraju_scc();
        assert_eq!(scc.len(), 1);
    }
}
