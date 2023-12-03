#define ARR_LENGTH 12

void swap(int* x1, int* x2);

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
        
    for (int index = 0; index < ARR_LENGTH - 1; index++)
    {
        // Assume the first element in the unsorted part is the minimum
        int currentMinValue = arr[index];
        int currentMinIndex = index;

        // Iterate over the unsorted part of the array
        for (int compareIndex = index + 1; compareIndex < ARR_LENGTH; compareIndex++)
        {
            // If a smaller element is found, update currentMinValue and currentMinIndex
            if (currentMinValue > arr[compareIndex])
            {
                currentMinValue = arr[compareIndex];
                currentMinIndex = compareIndex;
            }
        }
        // Swap the found minimum element with the first element in the unsorted part
        swap(&arr[index], &arr[currentMinIndex]);
    }
}

void swap(int* x1, int* x2)
{
    int temp = *x1;
    *x1 = *x2;
    *x2 = temp;
}
