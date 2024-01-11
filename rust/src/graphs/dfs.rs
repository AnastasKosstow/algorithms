
/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    DFS traversal sequence: A -> C -> D -> B
*/

use std::collections::HashSet;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Clone + Copy + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn dfs(&self, root_node: &Node<N>) -> Vec<N> {

        let mut result: Vec<N> = Vec::new();
        let mut stack: Vec<&NodeIndex> = vec![];
        stack.push(&root_node.index);

        let mut visited: HashSet<&NodeIndex> = HashSet::new();

        while let Some(node_index) = stack.pop() {
            if visited.contains(node_index) {
                continue;
            }
            visited.insert(node_index);
            result.push(self.nodes[node_index.ix].value);

            let edges = match self.edges.get(node_index) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if !visited.contains(&edge.node_index) {
                    stack.push(&edge.node_index);
                }
            }
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use crate::graphs::GraphType;

    use super::*;

    fn create_test_graph() -> Graph<char> {
        let mut graph = Graph::new(GraphType::Directed);
        let a = graph.add_node('A');
        let b = graph.add_node('B');
        let c = graph.add_node('C');
        let d = graph.add_node('D');

        graph.add_edge(a, b, 1);
        graph.add_edge(a, c, 1);
        graph.add_edge(b, d, 1);
        graph.add_edge(c, d, 1);

        graph
    }

    #[test]
    fn test_dfs_simple_graph() {
        let graph = create_test_graph();
        let root = graph.get_node(NodeIndex { ix: 0 }).unwrap();

        let traversal = graph.dfs(root);
        assert_eq!(traversal, vec!['A', 'C', 'D', 'B']);
    }

    #[test]
    fn test_dfs_empty_graph() {
        let graph: Graph<char> = Graph::new(GraphType::Directed);

        if let Some(root) = graph.nodes.get(0) {
            let traversal = graph.dfs(root);
            assert!(traversal.is_empty());
        }
    }
}