#![allow(dead_code)]
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut index: usize = 0;
    while index < arr.len() {
        let mut min_index: usize = index;

        for compare_index in index+1..arr.len() {
            if arr[min_index] > arr[compare_index] {
                min_index = compare_index;
            }
        }

        arr.swap(index, min_index);
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();

        selection_sort(&mut empty_vec);
        
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![3, -2, 9, 0, 12, -5, 8, 0];

        selection_sort(&mut arr);
        
        assert_eq!(arr, vec![-5, -2, 0, 0, 3, 8, 9, 12]);
    }
}
