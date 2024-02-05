use std::ptr::NonNull;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct TreeNode<T> {
    value: T,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>,
    parent: Option<NonNull<TreeNode<T>>>,
    balance_factor: i32
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct AvlTree<T> {
    root: Option<NonNull<TreeNode<T>>>,
    pub length: u32
}

impl<T> TreeNode<T> {
    fn new(value: T, parent: Option<NonNull<TreeNode<T>>>) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            parent: parent,
            balance_factor: 0
        }
    }
}

impl<T: std::fmt::Debug + Ord + PartialOrd + Copy> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            length: 0
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut inserted: bool = true;
        if let Some(mut root) = self.root {
            unsafe {
                inserted = insert_node(&mut root, value);
            }
        } else {
            let root_box = Box::new(TreeNode::<T>::new(value, None));
            let root_ptr = NonNull::new(Box::into_raw(root_box));
            self.root = root_ptr;
        }

        if inserted {
            self.length += 1;
        }

        inserted
    }
}

unsafe fn insert_node<T: Ord + PartialOrd>(node: &mut NonNull<TreeNode<T>>, value: T) -> bool {
    let mut_ptr = node.as_mut();
    if mut_ptr.value == value {
        return false;
    }

    let (target_node, balance_change) = if value < mut_ptr.value {
        (&mut mut_ptr.left, 1)
    } else {
        (&mut mut_ptr.right, -1)
    };

    let inserted = if let Some(ptr) = target_node {
        insert_node(ptr, value)
    } else {
        let node_box = Box::new(TreeNode::<T>::new(value, Some(*node)));
        let node_ptr = NonNull::new(Box::into_raw(node_box));
        *target_node = node_ptr;

        retrace(mut_ptr, balance_change);
        true
    };

    inserted
}

unsafe fn retrace<T: Ord + PartialOrd>(node: &mut TreeNode<T>, balance_change: i32) {
    node.balance_factor += balance_change;
    if node.balance_factor.abs() > 1 {
        // TODO: 
    } else if node.balance_factor != 0 && node.parent.is_some() {
        if let Some(mut parent) = node.parent {
            let parent_ptr = parent.as_mut();
            let balance_change = if parent_ptr.value > node.value { 1 } else { -1 };
            retrace(parent_ptr, balance_change);
        }
    }
}
