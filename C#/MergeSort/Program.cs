int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
arr = Sort(arr);

int[] Sort(int[] sourceArray)
{
    // Base case: if the array has only one element, it's already sorted
    if (sourceArray.Length == 1)
    {
        return sourceArray;
    }

    // Finding the middle index of the array to split it into two halves
    int middleIndex = sourceArray.Length / 2;

    // Create two sub-arrays from the original array and copy values
    // for example: if sourceArray is { 1, 2, 3, 4 } after Array.Copy operations leftArray is { 1, 2 } and rightArray is { 4, 3 }
    var leftArray = new int[middleIndex];
    var rightArray = new int[sourceArray.Length - middleIndex];

    Array.Copy(sourceArray, 0, leftArray, 0, middleIndex);
    Array.Copy(sourceArray, middleIndex, rightArray, 0, sourceArray.Length - middleIndex);

    // Use recursion to sort the left and right sub-arrays
    leftArray = Sort(leftArray);
    rightArray = Sort(rightArray);

    // After the two sub-arrays are sorted, Merge() method will merge/combine sub-arrays into one array
    return Merge(leftArray, rightArray);
}

int[] Merge(int[] leftArray, int[] rightArray)
{
    int leftArrayIndex = 0;
    int rightArrayIndex = 0;
    int mergedArrayIndex = 0;

    var mergedArray = new int[leftArray.Length + rightArray.Length];

    // Merging the two arrays until one of them is fully traversed
    while (leftArrayIndex < leftArray.Length && rightArrayIndex < rightArray.Length)
    {
        // Comparing elements of the two arrays and adding the smaller one to the merged array
        if (leftArray[leftArrayIndex] <= rightArray[rightArrayIndex])
        {
            mergedArray[mergedArrayIndex] = leftArray[leftArrayIndex];
            leftArrayIndex++;
        }
        else
        {
            mergedArray[mergedArrayIndex] = rightArray[rightArrayIndex];
            rightArrayIndex++;
        }
        mergedArrayIndex++;
    }

    // Adding remaining elements from the left array (if any)
    while (leftArrayIndex < leftArray.Length)
    {
        mergedArray[mergedArrayIndex] = leftArray[leftArrayIndex];
        leftArrayIndex++;
        mergedArrayIndex++;
    }

    // Adding remaining elements from the right array (if any)
    while (rightArrayIndex < rightArray.Length)
    {
        mergedArray[mergedArrayIndex] = rightArray[rightArrayIndex];
        rightArrayIndex++;
        mergedArrayIndex++;
    }

    return mergedArray;
}
