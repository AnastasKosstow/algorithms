use std::collections::HashSet;

use crate::graphs::graph::{ Graph, NodeIndex };

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn kosaraju_scc(&self) -> Vec<Vec<NodeIndex>> {
        if self.nodes.len() == 0 {
            return vec![];
        }

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut stack: Vec<NodeIndex> = Vec::new();

        for node in self.nodes.iter() {
            if !visited.contains(&node.index) {
                self.initial_dfs(&node.index, &mut visited, &mut stack);
            }
        }

        visited.clear();
        let mut components: Vec<Vec<NodeIndex>> = Vec::new();

        while let Some(node_index) = stack.pop() {
            if !visited.contains(&node_index) {
                let mut component = Vec::new();
                self.dfs_scc(&node_index, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    fn initial_dfs(&self, node_index: &NodeIndex, visited: &mut HashSet<NodeIndex>, stack: &mut Vec<NodeIndex>) {
        if visited.contains(&node_index) {
            return;
        }

        visited.insert(*node_index);
        
        if let Some(edges) = self.edges.get(&node_index) {
            for edge in edges {
                self.initial_dfs(&edge.node_index, visited, stack);
            }
        }
        
        stack.push(*node_index);
    }

    fn dfs_scc(&self, node_index: &NodeIndex, visited: &mut HashSet<NodeIndex>, component: &mut Vec<NodeIndex>) {
        if visited.contains(&node_index) {
            return;
        }

        visited.insert(*node_index);
        
        if let Some(edges) = self.edges.get(&node_index) {
            for edge in edges {
                self.dfs_scc(&edge.node_index, visited, component);
            }
        }
        
        component.push(*node_index);
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::graph::{Graph, GraphType, Node, NodeIndex, Edge};

    fn create_test_graph(edges: Vec<(usize, usize)>) -> Graph<i32> {
        let mut graph = Graph::new(GraphType::Directed);

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
