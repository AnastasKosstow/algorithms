use std::ptr::NonNull;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy)]
struct TreeNode<T> {
    value: T,
    color: Color,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>,
    height: usize
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct RedBlackTree<T> {
    root: Option<NonNull<TreeNode<T>>>,
    length: usize
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            color: Color::Red,
            left: None,
            right: None,
            height: 1
        }
    }

    fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    fn check_color_match(&self) -> Option<Side> {
        let left_match = self.left.as_ref().map_or(false, |&left_non_null| unsafe { 
            self.color == left_non_null.as_ref().color 
        });

        if left_match {
            return Some(Side::Left);
        }
        
        let right_match = self.right.as_ref().map_or(false, |&right_non_null| unsafe { 
            self.color == right_non_null.as_ref().color 
        });

        if right_match {
            return Some(Side::Right);
        }

        None // no rotations are needed
    }
}

impl<T: Ord + PartialOrd + Copy> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {
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
            let mut root = TreeNode::<T>::new(value);
            root.change_color(Color::Black);

            let root_box = Box::new(root);
            let root_ptr = NonNull::new(Box::into_raw(root_box));
            self.root = root_ptr;
            self.length += 1;
        }
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
    
    let color_match_side = node.check_color_match();
    if color_match_side.is_some() {
        // rotate
    }
}
