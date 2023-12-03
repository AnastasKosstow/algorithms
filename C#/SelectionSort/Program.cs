int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };

// Iterate over the array. Each iteration finds the minimum element in the unsorted part
for (int index = 0; index < arr.Length - 1; index++)
{
    // Assume the first element in the unsorted part is the minimum
    var currentMinValue = arr[index];
    var currentMinIndex = index;

    // Iterate over the unsorted part of the array
    for (int compareIndex = index + 1; compareIndex < arr.Length; compareIndex++)
    {
        if (currentMinValue > arr[compareIndex])
        {
            currentMinValue = arr[compareIndex];
            currentMinIndex = compareIndex;
        }
    }
    Swap(ref arr[index], ref arr[currentMinIndex]);
}

void Swap(ref int x1, ref int x2) => (x2, x1) = (x1, x2);
