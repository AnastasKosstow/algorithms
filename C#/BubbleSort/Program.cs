int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };

bool sorted;

// Initialize the index for the last sorted element to the array's length
// In bubbleSort after every iteration, the largest element in the unsorted part will move to its correct position at the end
// So after each iteration we do not need to check - arr.Length - index
int indexOfLastSortedElement = arr.Length;
do
{
    // Assume the array is sorted, change the value to false if a swap occurs
    sorted = true;
    for (int index = 1; index < indexOfLastSortedElement; index++)
    {
        // Compare each pair of elements
        // if they are out of order, swap
        if (arr[index - 1] > arr[index])
        {
            Swap(ref arr[index - 1], ref arr[index]);

            // Since a swap is made, mark the array as unsorted
            sorted = false;
        }
    }
    // Decrease the index of the last unsorted element by one
    // As the largest element in the unsorted part has moved to its correct position at the end
    indexOfLastSortedElement--;

} while (!sorted); // Repeat until no swaps occurs and sorted variable stay as true

void Swap(ref int x1, ref int x2) => (x2, x1) = (x1, x2);
