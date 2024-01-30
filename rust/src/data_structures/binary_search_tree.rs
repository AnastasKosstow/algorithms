use std::ptr::NonNull;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct TreeNode<T> {
    value: T,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct BinarySearchTree<T> {
    root: Option<NonNull<TreeNode<T>>>,
    pub length: u32
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None
        }
    }
}

impl<T: Ord + PartialOrd + Copy> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            length: 0
        }
    }

    pub fn insert(&mut self, value: T) {
        if let Some(root) = self.root {
            unsafe {
                insert_value(&mut (*root.as_ptr()), value);
                self.length += 1;
            }
        } else {
            let root_box = Box::new(TreeNode::<T>::new(value));
            let root_ptr = NonNull::new(Box::into_raw(root_box));
            self.root = root_ptr;
            self.length += 1;
        }
    }

    pub fn delete(&mut self, value: T) {
        if let Some(mut root) = self.root {
            unsafe {
                self.root = delete_node(&mut root, value);
                self.length -= 1;
            }
        }
    }

    pub fn get_all(&self) -> Vec<T> {
        let mut all = Vec::new();
        if let Some(root) = self.root {
            unsafe {
                get_all_in_order(&root, &mut all);
            }
        }
        all
    }

    pub fn clear(&mut self) {
        unsafe {
            if let Some(root) = self.root {
                clear_all_nodes(&root);
            }
        }
        
        self.root = None;
        self.length = 0;
    }
}


unsafe fn insert_value<T: Ord + PartialOrd>(node: &mut TreeNode<T>, value: T) {

    let target_node = 
        if value < node.value { 
            &mut node.left 
        } else { 
            &mut node.right 
        };

    match target_node {
        Some(ptr) => insert_value(&mut (*ptr.as_ptr()), value),
        None => {
            let node_box = Box::new(TreeNode::<T>::new(value));
            let node_ptr = NonNull::new(Box::into_raw(node_box));
            *target_node = node_ptr;
        }
    }
}

unsafe fn get_all_in_order<T: Copy>(node: &NonNull<TreeNode<T>>, all: &mut Vec<T>) {

    let tree_node = &(*node.as_ptr());

    if let Some(left_node) = tree_node.left {
        get_all_in_order(&left_node, all);
    }

    all.push(tree_node.value);

    if let Some(right_node) = tree_node.right {
        get_all_in_order(&right_node, all);
    }
}

unsafe fn clear_all_nodes<T>(node: &NonNull<TreeNode<T>>) {
    let boxed_node = Box::from_raw(node.as_ptr());
        
    if let Some(left_node) = boxed_node.left {
        clear_all_nodes(&left_node);
    }

    if let Some(right_node) = boxed_node.right {
        clear_all_nodes(&right_node);
    }
}

unsafe fn delete_node<T: Ord + PartialOrd + Copy>(node: &mut NonNull<TreeNode<T>>, value: T) -> Option<NonNull<TreeNode<T>>> {
    let tree_node = &mut *node.as_ptr();

    if value < tree_node.value {
        if let Some(mut left_node) = tree_node.left {
            tree_node.left = delete_node(&mut left_node, value);
        }
    } else if value > tree_node.value {
        if let Some(mut right_node) = tree_node.right {
            tree_node.right = delete_node(&mut right_node, value);
        }
    } else {
        if tree_node.left.is_none() {
            return tree_node.right;
        } else if tree_node.right.is_none() {
            return tree_node.left;
        }

        let mut successor = min_value_node(tree_node.right.unwrap());
        tree_node.value = (*successor.as_ptr()).value;
        tree_node.right = delete_node(&mut successor, tree_node.value);
    }

    Some(*node)
}

unsafe fn min_value_node<T>(node: NonNull<TreeNode<T>>) -> NonNull<TreeNode<T>> {
    let mut current = node;
    while let Some(left) = (*current.as_ptr()).left {
        current = left;
    }
    current
}
