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

// alternatively we can use 'rotate_right' function from rust std
/*
    the main idea is only to find the index at which we need to put 'current_item'
    and let 'rotate_right' to do all the swaps

    we find the correct position for 'current_item' without making any swaps
    and what 'rotate_right(1)' do is:
        shift every item one position right and the rightmost items goes to first position

    for example:
    
    arr = [1, 2, 3]
    index = 2
    insertion_index = 0

    arr[insertion_index..=index].rotate_right(1);
    arr = [3, 1, 2]

*/
pub fn insertion_sort_with_rotate_right<T: Ord + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    for index in 1..arr.len() {
        let current_item: T = arr[index];
        let mut insertion_index: usize = index;

        while insertion_index > 0 && arr[insertion_index - 1] > current_item {
            insertion_index -= 1;
        }
        arr[insertion_index..=index].rotate_right(1);
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
