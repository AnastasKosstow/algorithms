use crate::data_structures::{Heap, HeapType};

pub fn heap_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    
    let mut heap: Heap<T> = Heap::<T>::new(HeapType::Min);
    let length = arr.len();

    for item in arr.iter() {
        heap.push(*item);
    }

    for index in 0..length {
        if let Some(item) = heap.pop() {
            arr[index] = item;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();
        heap_sort(&mut empty_vec);

        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![4, -3, 7, 0, 10, -6, 7, 1];
        heap_sort(&mut arr);
        
        assert_eq!(arr, vec![-6, -3, 0, 1, 4, 7, 7, 10]);
    }
}
