int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };

for (int index = 0; index < arr.Length - 1; index++)
{
    int swapIndex = index;
    while (arr[swapIndex] > arr[swapIndex + 1])
    {
        if (swapIndex < 0) break;

        Swap(ref arr[swapIndex], ref arr[swapIndex + 1]);
        swapIndex--;
    }
}

void Swap(ref int x1, ref int x2)
    =>
    (x2, x1) = (x1, x2);
