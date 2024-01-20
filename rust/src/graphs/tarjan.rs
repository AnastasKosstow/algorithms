use std::collections::{HashSet, HashMap};

use crate::graphs::graph::{ Graph, NodeIndex };

use super::Node;

struct State {
    articulation_points: HashSet<NodeIndex>,
    nodes_discovery_time: HashMap<NodeIndex, i32>,
    nodes_low_link_value: HashMap<NodeIndex, i32>,
    parent_node: HashMap<NodeIndex, NodeIndex>,
    visited: HashSet<NodeIndex>,
    discovery_time: i32,
}

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn find_articulation_points(&self) -> HashSet<NodeIndex> {
        if self.length == 0 {
            return HashSet::new();
        }
    
        let mut state = State {
            articulation_points: HashSet::new(),
            nodes_discovery_time: HashMap::new(),
            nodes_low_link_value: HashMap::new(),
            parent_node: HashMap::new(),
            visited: HashSet::new(),
            discovery_time: 0
        };
    
        let root: &Node<N> = self.nodes.first().unwrap();
        self.tarjan_scc_dfs(root.index, &mut state, true);

        state.articulation_points
    }

    fn tarjan_scc_dfs(&self, node_index: NodeIndex, state: &mut State, is_root: bool) {
        state.discovery_time += 1;
        state.visited.insert(node_index);
        state.nodes_discovery_time.insert(node_index, state.discovery_time);
        state.nodes_low_link_value.insert(node_index, state.discovery_time);
    
        let mut children_count = 0; // Count the children of the node
    
        let edges = match self.edges.get(&node_index) {
            Some(edges) => edges,
            None => return
        };
    
        for edge in edges {
            if !state.visited.contains(&edge.node_index) {
                children_count += 1;
                state.parent_node.insert(edge.node_index, node_index);
                self.tarjan_scc_dfs(edge.node_index, state, false); // Not a root node
    
                state.nodes_low_link_value.insert(
                    node_index,
                    std::cmp::min(
                        state.nodes_low_link_value[&node_index],
                        state.nodes_low_link_value[&edge.node_index],
                    ),
                );
    
                if (!is_root && 
                    state.nodes_low_link_value[&edge.node_index] >= state.nodes_discovery_time[&node_index]) || (is_root && children_count > 1) {
                    state.articulation_points.insert(node_index);
                }
            } else if Some(&node_index) != state.parent_node.get(&edge.node_index) {
                state.nodes_low_link_value.insert(
                    node_index,
                    std::cmp::min(
                        state.nodes_low_link_value[&node_index],
                        state.nodes_discovery_time[&edge.node_index],
                    ),
                );
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::graph::{ Graph, GraphType };

    #[test]
    fn empty_graph() {
        let graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let ap = graph.find_articulation_points();
        assert!(ap.is_empty());
    }

    #[test]
    fn articulation_point_is_root() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Undirected);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");

        graph.add_edge(a, b, 1);
        graph.add_edge(a, c, 1);

        let ap = graph.find_articulation_points();

        assert_eq!(ap.len(), 1);
        assert!(ap.contains(&a));
    }

    #[test]
    fn find_two_articulation_points() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Undirected);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");
        let e = graph.add_node("E");
        let f = graph.add_node("F");
        let g = graph.add_node("G");

        graph.add_edge(a, b, 1);
        graph.add_edge(b, c, 1);
        graph.add_edge(c, d, 1);
        graph.add_edge(d, a, 1);
        graph.add_edge(c, g, 1);
        graph.add_edge(d, e, 1);
        graph.add_edge(e, f, 1);
        graph.add_edge(f, d, 1);

        let ap = graph.find_articulation_points();

        assert_eq!(ap.len(), 2);
        assert!(ap.contains(&c));
        assert!(ap.contains(&d));
    }
}
