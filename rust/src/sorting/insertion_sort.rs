fn insertion_sort() {
    let mut arr: [i32; 12] = [1, 0, 3, 2, 9, 5, 4, 0, 7, 3, 8, 6];

    for index in 1..arr.len() {
        let current_item: i32 = arr[index];
        let mut insertion_index: usize = index;

        while insertion_index > 0 && arr[insertion_index - 1] > current_item {
            arr[insertion_index] = arr[insertion_index - 1];
            insertion_index -= 1;
        }
        arr[insertion_index] = current_item;
    }
}
