use super::{tarjan_state::State, Graph, Node, NodeIndex};

struct SubGraphComponents {
    edge_stack: Vec<NodeIndex>,
    components: Vec<Vec<NodeIndex>>
}

impl<N: Ord + Eq + PartialEq + std::hash::Hash + Clone> Graph<N> {
    pub fn find_subgraph_components(&self) -> Vec<Vec<NodeIndex>> {
        if self.length == 0 {
            return Vec::new();
        }

        let mut state = State::new();
        let mut sgc = SubGraphComponents {
            edge_stack: Vec::new(),
            components: Vec::new()
        };
    
        let root: &Node<N> = self.nodes.first().unwrap();
        self.tarjan_sgc_dfs(root.index, &mut state, &mut sgc, true);
        sgc.components
    }

    fn tarjan_sgc_dfs
    (
        &self, 
        node_index: NodeIndex, 
        state: &mut State, 
        sgc: &mut SubGraphComponents, 
        is_root: bool
    ) {
        state.discovery_time += 1;
        state.visited.insert(node_index);
        state.nodes_discovery_time.insert(node_index, state.discovery_time);
        state.nodes_low_link_value.insert(node_index, state.discovery_time);

        sgc.edge_stack.push(node_index);
    
        let edges = match self.edges.get(&node_index) {
            Some(edges) => edges,
            None => return
        };
    
        for edge in edges {
            
            if !state.visited.contains(&edge.node_index) {
                state.parent_node.insert(edge.node_index, node_index);
                
                self.tarjan_sgc_dfs(edge.node_index, state, sgc, false); // Not a root node
    
                state.nodes_low_link_value.insert(
                    node_index,
                    std::cmp::min(
                        state.nodes_low_link_value[&node_index],
                        state.nodes_low_link_value[&edge.node_index],
                    ),
                );
    
                if (!is_root && 
                    state.nodes_low_link_value[&edge.node_index] >= state.nodes_discovery_time[&node_index]) || is_root {

                        let mut component = Vec::new();
                        while let Some(ix) = sgc.edge_stack.pop() {
                            component.push(ix);
                            if ix == node_index {
                                sgc.edge_stack.push(ix);
                                break;
                            }
                        }
                        sgc.components.push(component);
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
        let sub_graphs = graph.find_subgraph_components();
        assert!(sub_graphs.is_empty());
    }

    #[test]
    fn single_component() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Undirected);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");

        graph.add_edge(a, b, 1);
        graph.add_edge(b, c, 1);
        graph.add_edge(c, a, 1);

        let sub_graphs = graph.find_subgraph_components();

        assert_eq!(sub_graphs.len(), 1);
    }

    #[test]
    fn two_components_with_common_node() {
        let mut graph: Graph<&str> = Graph::new(GraphType::Undirected);
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");
        let e = graph.add_node("E");

        graph.add_edge(a, b, 1);
        graph.add_edge(b, c, 1);
        graph.add_edge(c, a, 1);
        graph.add_edge(c, d, 1);
        graph.add_edge(d, e, 1);
        graph.add_edge(e, c, 1);

        let sub_graphs = graph.find_subgraph_components();

        assert_eq!(sub_graphs.len(), 2);
        assert!(sub_graphs[0].contains(&c));
        assert!(sub_graphs[1].contains(&c));
    }
}
