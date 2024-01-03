#![allow(dead_code)]
use std::ptr::NonNull;

#[derive(Clone, Copy)]
pub struct Node<N> {
    pub value: N,
    pub next: Option<NonNull<Node<N>>>
}

pub struct LinkedList<N> {
    pub head: Option<NonNull<Node<N>>>,
    pub tail: Option<NonNull<Node<N>>>,
    pub length: usize,
}

impl<N: Clone> LinkedList<N> {
    pub fn new() -> Self {
        LinkedList { 
            head: None, 
            tail: None, 
            length: 0 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length > 0
    }

    pub fn add(&mut self, value: N) {
        let node: Node<N> = Node { 
            value, 
            next: None 
        };

        unsafe {
            self.add_node(node);
        }
        self.length += 1;
    }

    unsafe fn add_node(&mut self, node: Node<N>) {
        let node_box = Box::new(node);
        let node_ptr = NonNull::new(Box::into_raw(node_box));

        match self.tail {
            Some(tail) => {
                (*tail.as_ptr()).next = node_ptr;
                self.tail = node_ptr;
            },
            None => {
                self.head = node_ptr;
                self.tail = node_ptr;
            }
        }
    }

    pub fn get_all(&self) -> Vec<N> {
        let node: Option<NonNull<Node<N>>> = self.head;

        let result: Vec<N>;
        unsafe {
            match self.get_all_nodes_values(node) {
                Some(v) => result = v,
                None => result = vec![]
            };
        }
        result
    }

    unsafe fn get_all_nodes_values(&self, mut node: Option<NonNull<Node<N>>>) -> Option<Vec<N>> {
        let mut vec: Vec<N> = vec![];
        loop {
            match node {
                Some(n) => {
                    let current_node = &(*n.as_ptr());
                    vec.push(current_node.value.clone());
                    node = current_node.next;
                },
                None => break
            }
        }
        Some(vec)
    }
}
