#![allow(dead_code)]
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    for index in 1..arr.len() {
        let current_item: T = arr[index];
        let mut insertion_index: usize = index;

        while insertion_index > 0 && arr[insertion_index - 1] > current_item {
            arr[insertion_index] = arr[insertion_index - 1];
            insertion_index -= 1;
        }
        arr[insertion_index] = current_item;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();
        insertion_sort(&mut empty_vec);
        
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![4, -3, 7, 0, 10, -6, 7, 1];
        insertion_sort(&mut arr);
        
        assert_eq!(arr, vec![-6, -3, 0, 1, 4, 7, 7, 10]);
    }
}
