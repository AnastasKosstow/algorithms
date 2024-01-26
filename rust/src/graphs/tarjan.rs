use std::collections::HashMap;

use crate::graphs::graph::{ Graph, NodeIndex };

use super::{tarjan_state::State, Node};

/*
    NOT READY!
*/

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn find_low_link_values(&self) -> HashMap<NodeIndex, i32> {
        if self.length == 0 {
            return HashMap::new();
        }
    
        let mut state = State::new();
        let root: &Node<N> = self.nodes.first().unwrap();

        self.tarjan_dfs(root.index, &mut state);
        state.nodes_low_link_value
    }

    fn tarjan_dfs(&self, node_index: NodeIndex, state: &mut State) {
        state.discovery_time += 1;
        state.visited.insert(node_index);
        state.nodes_discovery_time.insert(node_index, state.discovery_time);
        state.nodes_low_link_value.insert(node_index, state.discovery_time);
    
        let edges = match self.edges.get(&node_index) {
            Some(edges) => edges,
            None => return
        };
    
        for edge in edges {
            if !state.visited.contains(&edge.node_index) {
                state.parent_node.insert(edge.node_index, node_index);
                self.tarjan_dfs(edge.node_index, state);
    
                state.nodes_low_link_value.insert(
                    node_index,
                    std::cmp::min(
                        state.nodes_low_link_value[&node_index],
                        state.nodes_low_link_value[&edge.node_index],
                    ),
                );
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
        let llv = graph.find_low_link_values();
        assert!(llv.is_empty());
    }

}
