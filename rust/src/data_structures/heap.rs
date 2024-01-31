#[derive(PartialEq)]
pub enum HeapType {
    Min,
    Max
}

pub struct Heap<T> {
    data: Vec<T>,
    heap_type: HeapType
}

impl<T: Ord + Clone + Copy> Heap<T> {
    pub fn new(heap_type: HeapType) -> Self {
        Self {
            data: vec![],
            heap_type
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn push(&mut self, item: T) {
        let old_len = self.data.len();
        self.data.push(item);
        self.sift_up(old_len);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.data.len() == 1 {
            let max = self.data.remove(self.data.len() - 1);
            return Some(max);
        }
        
        let max = self.data[0];
        let last = self.data.remove(self.data.len() - 1);
        self.data[0] = last;
        
        self.sift_down(0);
        Some(max)
    }

    fn sift_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }
    
        let parent_index = (index - 1) / 2;
        let should_swap = match self.heap_type {
            HeapType::Max => |a: &T, b: &T| a > b,
            HeapType::Min => |a: &T, b: &T| a < b,
        };
    
        if should_swap(&self.data[index], &self.data[parent_index]) {
            self.data.swap(index, parent_index);
            self.sift_up(parent_index);
        }
    }
    
    fn sift_down(&mut self, index: usize) {
        let left_child_index  = index * 2 + 1;
        let right_child_index  = index * 2 + 2;
        if !self.is_valid_index(left_child_index) {
            return;
        }
    
        let comparison = match self.heap_type {
            HeapType::Max => |a: &T, b: &T| a > b,
            HeapType::Min => |a: &T, b: &T| a < b,
        };
    
        let mut child_index = left_child_index;
        if self.is_valid_index(right_child_index) && comparison(&self.data[right_child_index], &self.data[left_child_index]) {
            child_index = right_child_index;
        }
    
        if comparison(&self.data[child_index], &self.data[index]) {
            self.data.swap(index, child_index);
            self.sift_down(child_index);
        }
    }
    
    fn is_valid_index(&self, index: usize) -> bool {
        index < self.data.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_min_heap() {
        let min_heap: Heap<i32> = Heap::new(HeapType::Min);
        assert!(min_heap.is_empty());
    }

    #[test]
    fn empty_max_heap() {
        let max_heap: Heap<i32> = Heap::new(HeapType::Max);
        assert!(max_heap.is_empty());
    }

    #[test]
    fn test_single_node_min_heap() {
        let mut min_heap = Heap::new(HeapType::Min);
        min_heap.push(10);
        assert_eq!(min_heap.pop(), Some(10));
        assert!(min_heap.is_empty());
    }

    #[test]
    fn test_single_node_max_heap() {
        let mut max_heap = Heap::new(HeapType::Max);
        max_heap.push(10);
        assert_eq!(max_heap.pop(), Some(10));
        assert!(max_heap.is_empty());
    }

    #[test]
    fn test_multiple_nodes_min_heap() {
        let mut min_heap = Heap::new(HeapType::Min);
        min_heap.push(10);
        min_heap.push(5);
        min_heap.push(15);
        assert_eq!(min_heap.pop(), Some(5));
        assert_eq!(min_heap.pop(), Some(10));
        assert_eq!(min_heap.pop(), Some(15));
    }

    #[test]
    fn test_multiple_nodes_max_heap() {
        let mut max_heap = Heap::new(HeapType::Max);
        max_heap.push(10);
        max_heap.push(5);
        max_heap.push(15);
        assert_eq!(max_heap.pop(), Some(15));
        assert_eq!(max_heap.pop(), Some(10));
        assert_eq!(max_heap.pop(), Some(5));
    }
}
