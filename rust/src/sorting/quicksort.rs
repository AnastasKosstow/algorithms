pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    sort(arr, 0, arr.len() - 1);
}

fn sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot_index = partition(arr, left, right);
        sort(arr, left, pivot_index);
        sort(arr, pivot_index + 1, right);
    }
}

fn partition<T: Ord>(arr: &mut [T], left: usize, right: usize) -> usize {
    let pivot = right;
    let mut p_left = left;
    let mut p_right = right - 1;

    loop {
        while arr[p_left] < arr[pivot] {
            p_left += 1;
        }

        while arr[p_right] > arr[pivot] {
            p_right -= 1;
        }

        if p_left >= p_right {
            arr.swap(p_left, pivot);
            return p_right;
        }
        arr.swap(p_left, p_right);
        p_left += 1;
        p_right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut empty_vec: Vec<i32> = Vec::new();
        quicksort(&mut empty_vec);
        
        assert!(empty_vec.is_empty());
    }

    #[test]
    fn sort() {
        let mut arr: Vec<i32> = vec![4, -3, 7, 0, 10, -6, 7, 1];
        quicksort(&mut arr);
        
        assert_eq!(arr, vec![-6, -3, 0, 1, 4, 7, 7, 10]);
    }
}
