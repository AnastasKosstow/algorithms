use std::collections::{HashMap, HashSet};

use super::NodeIndex;

pub struct State {
    pub nodes_discovery_time: HashMap<NodeIndex, i32>,
    pub nodes_low_link_value: HashMap<NodeIndex, i32>,
    pub parent_node: HashMap<NodeIndex, NodeIndex>,
    pub visited: HashSet<NodeIndex>,
    pub discovery_time: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            nodes_discovery_time: HashMap::new(),
            nodes_low_link_value: HashMap::new(),
            parent_node: HashMap::new(),
            visited: HashSet::new(),
            discovery_time: 0
        }
    }
}
