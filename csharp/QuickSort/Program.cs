int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
arr = Sort(arr, 0, arr.Length - 1);

T[] Sort<T>(T[] source, int low, int high) where T: IComparable<T>
{
    if (low < high)
    {
        // Partition the array segment and get the pivot's final position
        var pivot = Partition(source, low, high);

        // Recursively sort elements before the pivot
        Sort(source, low, pivot);

        // Recursively sort elements after the pivot
        Sort(source, pivot + 1, high);
    }
    return source;
}

int Partition<T>(T[] array, int left, int right) where T : IComparable<T>
{
    // Selecting the leftmost element as the pivot for simplicity.
    // The pivot is a reference point to divide the array into two parts.
    var pivot = array[left];
    while (true)
    {
        // With two while loops, one from left to pivot and one from right to pivot
        // we search for elements that we can swap
        // starting from left if we find element that is greater than pivot,
        // and fro right if we find element that is less than pivot,
        // we swap the two elements from left and right

        // Find the first element from the left that is greater than pivot
        while (array[left].CompareTo(pivot) < 0)
        {
            left++;
        }

        // Find the first element from the right that is less than pivot
        while (array[right].CompareTo(pivot) > 0)
        {
            right--;
        }

        // If all elements are compared and swapped if necessary, the partition is complete
        if (left >= right)
        {
            return right;
        }

        var temp = array[left];
        array[left] = array[right];
        array[right] = temp;

        // Move to next elements in the array for comparison
        left++;
        right--;
    }
}
