use std::{ops::Not, ptr::NonNull};

#[derive(Eq, PartialEq, Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Eq, PartialEq)]
enum Color {
    Red,
    Black,
}

struct TreeNode<T> {
    value: T,
    color: Color,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>,
    parent: Option<NonNull<TreeNode<T>>>,
    is_root: bool
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct RedBlackTree<T> {
    root: Option<NonNull<TreeNode<T>>>,
    length: usize
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
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
            is_root: false
        }
    }

    fn root(value: T) -> Self {
        TreeNode {
            value,
            color: Color::Black,
            left: None,
            right: None,
            parent: None,
            is_root: true
        }
    }

    fn change_color(&mut self) {
        self.color = match self.color {
            Color::Black => Color::Red,
            Color::Red => Color::Black
        };
    }

    fn child(&mut self, side: Side) -> &Option<NonNull<TreeNode<T>>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    fn child_mut(&mut self, side: Side) -> &mut Option<NonNull<TreeNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn check_color_match(&self) -> Option<Side> {
        let left_match = self
            .left
            .as_ref()
            .map_or(false, |&left_non_null| unsafe {
                self.color == Color::Red && self.color == left_non_null.as_ref().color
            });

        if left_match {
            return Some(Side::Left);
        }

        let right_match = self
            .right
            .as_ref()
            .map_or(false, |&right_non_null| unsafe {
                self.color == Color::Red && self.color == right_non_null.as_ref().color
            });

        if right_match {
            return Some(Side::Right);
        }

        None // no rotations are needed
    }
}

impl<T: Ord + PartialOrd> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            length: 0
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let inserted;
        if let Some(mut root) = self.root {
            unsafe {
                inserted = insert_value(&mut root, value);
                self.update_root();
                self.length += 1;
            }
        } else {
            let root_box = Box::new(TreeNode::<T>::root(value));
            let root_ptr = NonNull::new(Box::into_raw(root_box));
            self.root = root_ptr;
            self.length += 1;
            inserted = true;
        }
        inserted
    }

    fn update_root(&mut self) {
        unsafe {
            while let Some(mut root) = self.root {
                if let Some(mut parent) = root.as_ref().parent {
                    root.as_mut().is_root = false;
                    parent.as_mut().is_root = true;
                    self.root = Some(parent);
                } else {
                    break;
                }
            }
        }
    }
}


unsafe fn insert_value<T: Ord + PartialOrd>(node: &mut NonNull<TreeNode<T>>, value: T) -> bool {
    let node_ptr = node.as_mut();
    if node_ptr.value == value {
        return false; // Value already exists in the tree
    }

    let target_node = 
        if value < node_ptr.value { 
            &mut node_ptr.left 
        } else { 
            &mut node_ptr.right 
        };

    let inserted = match target_node {
        Some(mut ptr) => insert_value(&mut ptr, value),
        None => {
            let mut new_node = TreeNode::<T>::new(value);
            new_node.parent = Some(*node);

            let node_box = Box::new(new_node);
            let node_ptr = NonNull::new(Box::into_raw(node_box));
            *target_node = node_ptr;
            true
        }
    };

    let match_side = node.as_ref().check_color_match(); // Return 'Side' if node and new_node are with same 'Red' color
    if match_side.is_some() {
        // TODO:
    }
    
    inserted
}
