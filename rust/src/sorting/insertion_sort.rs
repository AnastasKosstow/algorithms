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
