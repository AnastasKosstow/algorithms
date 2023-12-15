fn merge_sort() {
    let mut arr: [i32; 12] = [1, 0, 3, 2, 9, 5, 4, 0, 7, 3, 8, 6];
    sort(&mut arr);
}

fn sort(arr: &mut [i32]) {
    if arr.len() == 1 {
        return; // If the array has only one element, it's already sorted
    } else {
        // Finding the middle index of the array to split it into two halves
        let middle_index: usize = arr.len() / 2;
        let (left_array, right_array) = arr.split_at_mut(middle_index);

        sort(left_array);
        sort(right_array);

        let merged = merge(left_array, right_array);
        arr.copy_from_slice(&merged);
    } 

    // Use recursion to sort the left and right sub-arrays

    // After the two sub-arrays are sorted, Merge() method will merge/combine sub-arrays into one array
}

fn merge(left_array: &[i32], right_array: &[i32]) -> Vec<i32> {
    
    let mut left_array_index: usize = 0;
    let mut right_array_index: usize = 0;
    let mut merged_vector: Vec<i32> = Vec::with_capacity(left_array.len() + right_array.len());

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
