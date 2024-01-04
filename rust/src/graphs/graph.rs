#![allow(dead_code)]

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

pub struct Edge {
    pub cost: usize,
    pub node_index: NodeIndex
}

pub struct Graph<N> {
    pub nodes: Vec<Node<N>>,
    pub edges: std::collections::HashMap<NodeIndex, Vec<Edge>>,
    pub ty: GraphType,
    pub lenght: usize
}

impl<N: Eq + std::hash::Hash + PartialEq> Graph<N> {
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            nodes: Vec::new(),
            edges: std::collections::HashMap::new(),
            ty: graph_type,
            lenght: 0
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
        self.lenght += 1;
        index
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, cost: usize) {
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
        assert_eq!(empty_graph.lenght, 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        assert_eq!(graph.lenght, 0);

        let node_index = graph.add_node(5);
        assert_eq!(graph.lenght, 1);
        assert_eq!(graph.get_node(node_index).unwrap().value, 5);
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);

        let edges = graph.edges.get(&node1).unwrap();
        assert_eq!(edges[0].cost, 10);
        assert_eq!(edges[0].node_index.ix, node2.ix);
    }

    #[test]
    fn test_directed_edges() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Directed);
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);

        graph.add_edge(node1, node2, 10);

        let edges = graph.edges.get(&node1).unwrap();
        assert_eq!(edges[0].cost, 10);
        assert_eq!(edges[0].node_index.ix, node2.ix);

        // For directed graphs, there should be no reverse edge
        assert!(graph.edges.get(&node2).is_none());
    }

    #[test]
    fn test_get_node() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        let node1 = graph.add_node(1);

        let retrieved_node = graph.get_node(node1).unwrap();
        assert_eq!(retrieved_node.value, 1);
    }

    #[test]
    fn test_invalid_node_index() {
        let graph: Graph<i32> = Graph::new(GraphType::Undirected);
        assert!(graph.get_node(NodeIndex { ix: 999 }).is_none());
    }

    #[test]
    fn test_graph_length() {
        let mut graph: Graph<i32> = Graph::new(GraphType::Undirected);
        graph.add_node(1);
        graph.add_node(2);

        assert_eq!(graph.lenght, 2);
    }
}