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

    fn child(&mut self, side: Side) -> &mut Option<NonNull<TreeNode<T>>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn check_color_match(&self) -> Option<Side> {
        let left_match = self.left.as_ref().map_or(false, |&left_non_null| unsafe {
            self.color == Color::Red && self.color == left_non_null.as_ref().color 
        });

        if left_match {
            return Some(Side::Left);
        }
        
        let right_match = self.right.as_ref().map_or(false, |&right_non_null| unsafe { 
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

    check_for_rebalancing(node_ptr);
    inserted
}

unsafe fn check_for_rebalancing<T: Ord + PartialOrd>(node: &mut TreeNode<T>) {
    let match_side = node.check_color_match(); // Return 'Side' if node and new_node are with same 'Red' color
    if match_side.is_some() {
        if let Some(mut grandparent) = node.parent {
            let left_child_red = grandparent
                .as_ref().left
                .map_or(false, |left_ptr| left_ptr.as_ref().color == Color::Red);

            let right_child_red = grandparent
                .as_ref().right
                .map_or(false, |right_ptr| right_ptr.as_ref().color == Color::Red);
        
            if left_child_red && right_child_red {
                grandparent.as_ref().left.unwrap().as_mut().change_color();
                grandparent.as_ref().right.unwrap().as_mut().change_color();
                if !grandparent.as_ref().is_root {
                    grandparent.as_mut().change_color();
                }
            }
            else {
                rotate(node, match_side.unwrap());
            }
        }
    }
}

unsafe fn rotate<T: Ord + PartialOrd>(node: &mut TreeNode<T>, side: Side) {
    if let Some(mut parent_node) = node.parent {
        let parent_node_mut: &mut TreeNode<T> = parent_node.as_mut();
        node.child(!side).unwrap().as_mut().parent = NonNull::new(parent_node_mut);

        *parent_node_mut.child(side) = node.child(!side).take();
        *node.child(!side) = node.parent;

        if let Some(mut opposite_node_child) = node.child(!side) {
            let grandparent = (*node.parent.unwrap().as_ptr()).parent;
            opposite_node_child.as_mut().parent = NonNull::new(node);
            node.parent = grandparent;

            node.change_color();
            opposite_node_child.as_mut().change_color();
        }
    }
}
