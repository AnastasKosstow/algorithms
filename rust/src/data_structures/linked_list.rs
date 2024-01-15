#![allow(dead_code)]
use std::ptr::NonNull;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Node<N> {
    pub value: N,
    pub next: Option<NonNull<Node<N>>>,
    prev: Option<NonNull<Node<N>>>
}

pub struct LinkedList<N> {
    pub head: Option<NonNull<Node<N>>>,
    pub tail: Option<NonNull<Node<N>>>,
    pub length: usize,
}

impl<N: Clone + Eq + PartialEq> LinkedList<N> {
    pub fn new() -> Self {
        LinkedList { 
            head: None, 
            tail: None, 
            length: 0 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn add(&mut self, value: N) {
        let node: Node<N> = Node { 
            value, 
            next: None,
            prev: self.tail
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

    pub fn delete(&mut self, value: N) {
        if self.is_empty() {
            return;
        }

        let node: Option<NonNull<Node<N>>> = self.head;
        unsafe {
            self.delete_node(value, node);
        }
    }

    unsafe fn delete_node(&mut self, value: N, mut node: Option<NonNull<Node<N>>>) {
        while let Some(current_node) = node {
            if current_node.as_ref().value == value {
                let old_ptr = Box::from_raw(current_node.as_ptr());

                if let Some(mut prev) = old_ptr.prev {
                    prev.as_mut().next = old_ptr.next;
                }

                if let Some(mut next) = old_ptr.next {
                    next.as_mut().prev = old_ptr.prev;
                }

                self.length -= 1;
                break;
            }
            node = (*current_node.as_ptr()).next;
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            self.clear_all_nodes();
        }
        
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    unsafe fn clear_all_nodes(&mut self) {
        while let Some(node) = self.head {
            let boxed_node = Box::from_raw(node.as_ptr());
            self.head = boxed_node.next;
        }
    }
}

impl<N> Drop for LinkedList<N> {
    fn drop(&mut self) {
        while let Some(node) = self.head {
            unsafe {
                let boxed_node = Box::from_raw(node.as_ptr());
                self.head = boxed_node.next;
            }
        }
    }
}
