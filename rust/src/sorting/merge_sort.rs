pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    sort(arr);
}

fn sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    if arr.len() == 1 {
        return; // If the array has only one element, it's already sorted
    }
    
    // Finding the middle index of the array to split it into two halves
    let middle_index: usize = arr.len() / 2;
    let (left_array, right_array) = arr.split_at_mut(middle_index);

    sort(left_array);
    sort(right_array);

    let merged = merge(left_array, right_array);
    for (index, item) in merged.iter().enumerate() {
        arr[index] = item.clone();
    }
}

fn merge<T: Ord + Clone + Copy>(left_array: &[T], right_array: &[T]) -> Vec<T> {    
    let mut left_array_index: usize = 0;
    let mut right_array_index: usize = 0;
    let mut merged_vector: Vec<T> = Vec::with_capacity(left_array.len() + right_array.len());

    // Merging the two arrays until one of them is fully traversed
    while left_array_index < left_array.len() && right_array_index < right_array.len() {
        if left_array[left_array_index] <= right_array[right_array_index] {
            merged_vector.push(left_array[left_array_index]);
            left_array_index += 1;
        } else {
            merged_vector.push(right_array[right_array_index]);
            right_array_index += 1;
        }
    }

    // Adding remaining elements from the left array (if any)
    while left_array_index < left_array.len() {
        merged_vector.push(left_array[left_array_index]);
        left_array_index += 1;
    }

    // Adding remaining elements from the right array (if any)
    while right_array_index < right_array.len() {
        merged_vector.push(right_array[right_array_index]);
        right_array_index += 1;
    }   

    merged_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();
        merge_sort(&mut empty_vec);
        
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![4, -3, 7, 0, 10, -6, 7, 1];
        merge_sort(&mut arr);
        
        assert_eq!(arr, vec![-6, -3, 0, 1, 4, 7, 7, 10]);
    }
}
