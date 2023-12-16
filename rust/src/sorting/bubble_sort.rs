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
