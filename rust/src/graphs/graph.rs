#![allow(dead_code)]

#[derive(Ord, PartialOrd, Eq, Hash, PartialEq, Clone, Copy)]
pub struct NodeIndex {
    ix: usize
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
    pub ty: GraphType
}

impl<N: Eq + std::hash::Hash + PartialEq> Graph<N> {
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            nodes: Vec::new(),
            edges: std::collections::HashMap::new(),
            ty: graph_type,
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
