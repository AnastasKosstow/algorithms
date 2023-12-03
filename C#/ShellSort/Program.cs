int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, };
int[] sequence = GenerateKnuthSequence(arr.Length);

// Iterate over the Knuth sequence
// and perform insertion sort for every step
for (int stepIndex = 0; stepIndex < sequence.Length; stepIndex++)
{
    int step = sequence[stepIndex];

    for (int index = step; index < arr.Length; index++)
    {
        var currentItem = arr[index];
        int compareIndex = index - step;

        // Shift elements that are step distance apart and greater than currentItem
        while (compareIndex >= 0 && currentItem < arr[compareIndex])
        {
            arr[compareIndex + step] = arr[compareIndex];
            compareIndex -= step;
        }

        // Place currentItem in its correct position
        arr[compareIndex + step] = currentItem;
    }
}

// Generate the Knuth sequence based on the array size
// 1, 4, 13, 40, 121, ...
static int[] GenerateKnuthSequence(int arraySize)
{
    int gapSequenceLength = (int)Math.Floor(Math.Log(arraySize, 3)) - 1;

    int[] knuthSequence = new int[gapSequenceLength];
    int value = 1;

    for (int index = 0; index < gapSequenceLength; index++)
    {
        knuthSequence[index] = (value);
        value = 3 * value + 1; // Calculate the next gap size
    }

    Array.Reverse(knuthSequence); // Reverse the sequence as per Knuth's suggestion
    return knuthSequence;
}
