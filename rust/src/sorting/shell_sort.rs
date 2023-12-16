pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let sequence: Vec<i32> = generate_knuth_sequence(arr.len());

    for seq_value in sequence.iter() {
        let seq_value = *seq_value as usize; // Dereference to get the i32 value

        for index in seq_value..arr.len() {
            let current_item: T = arr[index];
            let mut compare_index: usize = index;
    
            while compare_index >= seq_value && arr[compare_index - seq_value] > current_item {
                arr[compare_index] = arr[compare_index - seq_value];
                compare_index -= seq_value;
            }
            arr[compare_index] = current_item;
        }
    }
}

// Generate the Knuth sequence based on the array size
// 1, 4, 13, 40, 121, ...
fn generate_knuth_sequence(array_size: usize) -> Vec<i32> {
    let gap_sequence_length = (f64::log(array_size as f64, 3.0)).floor() as usize - 1;

    let mut knuth_sequence: Vec<i32> = Vec::with_capacity(gap_sequence_length);
    let mut value: i32 = 1;

    for _index in 0..gap_sequence_length {
        knuth_sequence.push(value);
        value = 3 * value + 1; // Calculate the next gap size
    }

    knuth_sequence.reverse(); // Reverse the sequence as per Knuth's suggestion
    knuth_sequence
}