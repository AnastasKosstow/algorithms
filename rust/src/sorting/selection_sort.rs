
fn selection_sort() {
    let mut arr: [i32; 12] = [1, 0, 3, 2, 9, 5, 4, 0, 7, 3, 8, 6];

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
