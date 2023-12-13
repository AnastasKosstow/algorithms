fn bubble_sort() {
    let mut arr: [i32; 12] = [1, 0, 3, 2, 9, 5, 4, 0, 7, 3, 8, 6];

    for index in (0..arr.len()).rev() {
        for sort_index in 0..index {
            if arr[sort_index] > arr[sort_index + 1] {
                arr.swap(sort_index, sort_index + 1)
            }
        }
    }
}
