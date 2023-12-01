#include <stdbool.h>

#define ARR_LENGTH 12

void swap(int* x1, int* x2);

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
        
    bool sorted = true;

    // Initialize the index for the last sorted element to the array's length
    // In bubbleSort after every iteration, the largest element in the unsorted part will move to its correct position at the end
    // So after each iteration we do not need to check - arr.Length - index
    int indexOfLastSortedElement = ARR_LENGTH;
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
                swap(&arr[index - 1], &arr[index]);

                // Since a swap is made, mark the array as unsorted
                sorted = false;
            }
        }
        // Decrease the index of the last unsorted element by one
        // As the largest element in the unsorted part has moved to its correct position at the end
        indexOfLastSortedElement--;

    } while (!sorted); // Repeat until no swaps occurs and sorted variable stay as true
}

void swap(int* x1, int* x2)
{
    int temp = *x1;
    *x1 = *x2;
    *x2 = temp;
}