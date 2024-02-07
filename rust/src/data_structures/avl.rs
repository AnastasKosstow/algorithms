use std::{cmp::max, mem, ops::Not, ptr::NonNull};

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct TreeNode<T> {
    value: T,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>,
    parent: Option<NonNull<TreeNode<T>>>,
    height: usize
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct AvlTree<T> {
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
    fn new(value: T, parent: Option<NonNull<TreeNode<T>>>) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
            parent: parent,
            height: 0
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
        if left < right {
            (self.height) as i8
        } else {
            -((self.height) as i8)
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }
}

impl<T: std::fmt::Debug + Ord + PartialOrd + Copy> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            length: 0
        }
    }

    pub fn print(&self) {
        let mut vec: Vec<String> = vec![];
        print_tree(self.root.unwrap(), &mut vec, 0);

        while let Some(item) = vec.pop() {
            println!("{:?}", item);
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

fn print_tree<T: std::fmt::Debug>(node: NonNull<TreeNode<T>>, stack: &mut Vec<String>, mut level: i32) {
    unsafe {
        let node_ptr = node.as_ref();
        if let Some(left) = node_ptr.left {
            level += 1;
            print_tree(left, stack, level);
            level -= 1;
        }
        if let Some(right) = node_ptr.right {
            level += 1;
            print_tree(right, stack, level);
            level -= 1;
        }
        let line: String = format!("item: {:?} - level: {}", node_ptr.value, level);
        stack.push(line);
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
            let node_box = Box::new(TreeNode::<T>::new(value, Some(*node)));
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


unsafe fn rebalance<T: Copy>(node: &mut NonNull<TreeNode<T>>) {
    let node_ptr = node.as_mut();
    node_ptr.update_height();

    let side = match node_ptr.balance_factor() {
        -2 => Side::Left,
        2 => Side::Right,
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
