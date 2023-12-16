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
