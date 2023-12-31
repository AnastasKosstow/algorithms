#![allow(dead_code)]
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    for index in (0..arr.len()).rev() {
        for sort_index in 0..index {
            if arr[sort_index] > arr[sort_index + 1] {
                arr.swap(sort_index, sort_index + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();

        bubble_sort(&mut empty_vec);
        
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];

        bubble_sort(&mut arr);
        
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}
