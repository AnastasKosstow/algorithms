use std::{cmp::max, mem, ops::Not, ptr::NonNull};

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<NonNull<TreeNode<T>>>,
    pub right: Option<NonNull<TreeNode<T>>>,
    height: usize
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct AvlTree<T> {
    pub root: Option<NonNull<TreeNode<T>>>,
    pub length: usize
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            height: 1
        }
    }

    fn child(&mut self, side: Side) -> &mut Option<NonNull<TreeNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn height(&mut self, side: Side) -> usize {
        self.child(side)
            .as_ref()
            .map_or(0, |n| {
                unsafe {
                    (*n.as_ptr()).height
                }
            })
    }

    fn balance_factor(&mut self) -> i8 {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));
        if left > right {
            (left - right) as i8
        } else {
            -((right - left) as i8)
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }
}

impl<T: Ord + PartialOrd + Copy> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            length: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut inserted: bool = true;
        if let Some(mut root) = self.root {
            unsafe {
                inserted = insert_node(&mut root, value);
            }
        } else {
            let root_box = Box::new(TreeNode::<T>::new(value));
            let root_ptr = NonNull::new(Box::into_raw(root_box));
            self.root = root_ptr;
        }

        if inserted {
            self.length += 1;
        }

        inserted
    }

    pub fn delete(&mut self, value: T) {
        if let Some(mut root) = self.root {
            unsafe {
                self.root = delete_node(&mut root, value);
                self.length -= 1;
            }
        }
    }
}


unsafe fn insert_node<T: Ord + PartialOrd + Copy>(node: &mut NonNull<TreeNode<T>>, value: T) -> bool {
    let node_ptr = node.as_mut();
    if node_ptr.value == value {
        return false; // Value already exists in the tree
    }

    let target_node = if value < node_ptr.value {
        &mut node_ptr.left
    } else {
        &mut node_ptr.right
    };

    let inserted = match target_node {
        Some(ptr) => {
            insert_node(ptr, value)
        },
        None => {
            let node_box = Box::new(TreeNode::<T>::new(value));
            let node_ptr = NonNull::new(Box::into_raw(node_box));
            *target_node = node_ptr;
            true
        },
    };

    if inserted {
        rebalance(node);
    }
    inserted
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

    rebalance(node);
    Some(*node)
}


unsafe fn min_value_node<T>(node: NonNull<TreeNode<T>>) -> NonNull<TreeNode<T>> {
    let mut current = node;
    while let Some(left) = (*current.as_ptr()).left {
        current = left;
    }
    current
}


unsafe fn rebalance<T: Copy>(node: &mut NonNull<TreeNode<T>>) {
    let node_ptr = node.as_mut();
    node_ptr.update_height();

    let side = match node_ptr.balance_factor() {
        2 => Side::Left,
       -2 => Side::Right,
        _ => return,
    };

    match node_ptr.child(side) {
        Some(subtree) => {
            if let (Side::Left, -1) | (Side::Right, 1) = (side, (subtree.as_mut()).balance_factor()) {
                rotate(subtree, side);
            }
            rotate(node, !side);
        },
        None => return
    }
}


unsafe fn rotate<T>(node: &mut NonNull<TreeNode<T>>, side: Side) {
    let node_ptr = node.as_mut();
    let mut subtree = node_ptr.child(!side).take().unwrap();
    *node_ptr.child(!side) = (subtree.as_mut()).child(side).take();
    node_ptr.update_height();
    
    mem::swap(node_ptr, subtree.as_mut());
    
    *node_ptr.child(side) = Some(subtree);
    node_ptr.update_height();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn single_element() {
        let mut tree = AvlTree::new();
        assert!(tree.insert(10));
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn left_left_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(30);
        tree.insert(20);

        unsafe {
            assert_eq!(tree.root.unwrap().as_ref().value, 30);
        }

        tree.insert(10);

        unsafe {
            let root = tree.root.unwrap();
            assert_eq!(root.as_ref().value, 20);
            assert_eq!(root.as_ref().left.unwrap().as_ref().value, 10);
            assert_eq!(root.as_ref().right.unwrap().as_ref().value, 30);
        }
    }

    #[test]
    fn left_right_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(30);
        tree.insert(20);

        unsafe {
            assert_eq!(tree.root.unwrap().as_ref().value, 30);
        }

        tree.insert(25);

        unsafe {
            let root = tree.root.unwrap();
            assert_eq!(root.as_ref().value, 25);
            assert_eq!(root.as_ref().left.unwrap().as_ref().value, 20);
            assert_eq!(root.as_ref().right.unwrap().as_ref().value, 30);
        }
    }

    #[test]
    fn right_right_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(20);

        unsafe {
            assert_eq!(tree.root.unwrap().as_ref().value, 10);
        }

        tree.insert(30);

        unsafe {
            let root = tree.root.unwrap();
            assert_eq!(root.as_ref().value, 20);
            assert_eq!(root.as_ref().left.unwrap().as_ref().value, 10);
            assert_eq!(root.as_ref().right.unwrap().as_ref().value, 30);
        }
    }

    #[test]
    fn right_left_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(20);

        unsafe {
            assert_eq!(tree.root.unwrap().as_ref().value, 10);
        }

        tree.insert(15);

        unsafe {
            let root = tree.root.unwrap();
            assert_eq!(root.as_ref().value, 15);
            assert_eq!(root.as_ref().left.unwrap().as_ref().value, 10);
            assert_eq!(root.as_ref().right.unwrap().as_ref().value, 20);
        }
    }

    #[test]
    fn delete_node() {
        let mut tree = AvlTree::new();
        tree.insert(20);
        tree.insert(10);
        tree.insert(30);
        tree.insert(40);

        unsafe {
            assert_eq!(tree.root.unwrap().as_ref().value, 20);
        }

        tree.delete(10);

        unsafe {
            let root = tree.root.unwrap();
            assert_eq!(root.as_ref().value, 30);
            assert_eq!(root.as_ref().left.unwrap().as_ref().value, 20);
            assert_eq!(root.as_ref().right.unwrap().as_ref().value, 40);
        }
    }
}
