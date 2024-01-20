#[derive(Ord, PartialOrd, Eq, Hash, PartialEq, Clone, Copy)]
pub struct NodeIndex {
    pub ix: usize
}

#[derive(Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Node<N> {
    pub value: N,
    pub index: NodeIndex
}

#[derive(PartialEq)]
pub enum GraphType {
    Directed,
    Undirected
}

#[derive(PartialEq)]
pub struct Edge {
    pub cost: isize,
    pub node_index: NodeIndex
}

pub struct Graph<N> {
    pub nodes: Vec<Node<N>>,
    pub edges: std::collections::HashMap<NodeIndex, Vec<Edge>>,
    pub ty: GraphType,
    pub length: usize
}

impl<N: Eq + std::hash::Hash + PartialEq> Graph<N> {
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            nodes: Vec::new(),
            edges: std::collections::HashMap::new(),
            ty: graph_type,
            length: 0
        }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        let index = NodeIndex { 
            ix: self.nodes.len() 
        };
        
        self.nodes.push(Node { 
            value: value,
            index: index
        });
        self.length += 1;
        index
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, cost: isize) {
        if from.ix >= self.nodes.len() || to.ix >= self.nodes.len() {
            return; // Invalid node index
        }

        self.edges.entry(from)
            .or_insert_with(Vec::new)
            .push(Edge {
                cost,
                node_index: to
            });

        if self.ty == GraphType::Undirected {
            self.edges.entry(to)
                .or_insert_with(Vec::new)
                .push(Edge {
                    cost,
                    node_index: from
                });
        }
    }

    pub fn delete_node(&mut self, node_index: NodeIndex) {
        for (_, edges) in self.edges.iter_mut() {
            edges.retain(|edge| edge.node_index != node_index);
        }
     
        self.edges.remove(&node_index);     
        self.nodes.remove(node_index.ix);

        for node in self.nodes.iter_mut() {
            if node.index.ix > node_index.ix {
                if let Some(edges) = self.edges.remove(&node.index) {
                    self.edges.insert(NodeIndex { 
                        ix: node.index.ix - 1 
                    }, 
                    edges);
                } 
                node.index.ix -= 1;
            }
        }

        self.length -= 1;
    }

    pub fn delete_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        if let Some(edges) = self.edges.get_mut(&from) {
            
            let pos = edges.iter().position(|edge| edge.node_index == to);
            if let Some(pos) = pos {
                edges.remove(pos);
            }
        }
    }

    pub fn get_node(&self, node_index: NodeIndex) -> Option<&Node<N>> {
        self.nodes.get(node_index.ix)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty_graph: Graph<i32> = Graph::new(GraphType::Undirected);
        assert_eq!(empty_graph.length, 0);
    }

    #[test]
    fn add_node() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        assert_eq!(graph.length, 0);

        let node_index = graph.add_node(5);
        assert_eq!(graph.length, 1);
        assert_eq!(graph.get_node(node_index).unwrap().value, 5);
    }

    #[test]
    fn add_edge() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);

        let edges = graph.edges.get(&node1).unwrap();
        assert_eq!(edges[0].cost, 10);
        assert_eq!(edges[0].node_index.ix, node2.ix);
    }

    #[test]
    fn directed_edges() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Directed);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);

        let edges = graph.edges.get(&node1).unwrap();
        assert_eq!(edges[0].cost, 10);
        assert_eq!(edges[0].node_index.ix, node2.ix);
        assert!(graph.edges.get(&node2).is_none());
    }

    #[test]
    fn delete_node() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);
        graph.delete_node(node1);

        assert!(graph.get_node(node2).is_none());
        assert_eq!(graph.get_node(node1).unwrap().value, 2);
        assert_eq!(graph.edges.get(&node1).unwrap().len(), 0);
        assert_eq!(graph.length, 1);
    }

    #[test]
    fn delete_edge() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);
        graph.delete_edge(node1, node2);

        let edges = graph.edges.get(&node1).unwrap();
        assert!(edges.is_empty());
    }

    #[test]
    fn get_node() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);

        let retrieved_node = graph.get_node(node1).unwrap();
        assert_eq!(retrieved_node.value, 1);
    }

    #[test]
    fn invalid_node_index() {
        let graph: Graph<i32> = Graph::new(GraphType::Undirected);
        assert!(graph.get_node(NodeIndex { ix: 999 }).is_none());
    }

    #[test]
    fn graph_length() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        graph.add_node(1);
        graph.add_node(2);

        assert_eq!(graph.length, 2);
    }
}