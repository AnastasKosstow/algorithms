
/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    BFS traversal sequence: A -> B -> C -> D
*/

use std::collections::VecDeque;

use crate::graphs::graph::{ Graph, Node, NodeIndex };

impl<N: Clone + Copy + Eq + PartialEq + std::hash::Hash> Graph<N> {
    pub fn bfs(&self, root_node: &Node<N>) -> Vec<N> {

        let mut result: Vec<N> = Vec::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        queue.push_back(root_node.index);

        let mut visited: Vec<NodeIndex> = Vec::new();

        while let Some(node_index) = queue.pop_front() {
            if visited.contains(&node_index) {
                continue;
            }

            visited.push(node_index);
            result.push(self.nodes[node_index.ix].value);

            let edges = match self.edges.get(&node_index) {
                Some(edges) => edges,
                None => continue
            };

            for edge in edges {
                if !visited.contains(&edge.node_index) && !queue.contains(&edge.node_index) {
                    queue.push_back(edge.node_index);
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
    fn bfs_simple_graph() {
        let graph = create_test_graph();
        let root = graph.get_node(NodeIndex { ix: 0 }).unwrap();

        let traversal = graph.bfs(root);
        assert_eq!(traversal, vec!['A', 'B', 'C', 'D']);
    }

    #[test]
    fn bfs_empty_graph() {
        let graph: Graph<char> = Graph::new(GraphType::Directed);

        if let Some(root) = graph.nodes.get(0) {
            let traversal = graph.bfs(root);
            assert!(traversal.is_empty());
        }
    }
}