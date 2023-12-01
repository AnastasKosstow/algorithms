#include <stdio.h>

#define ARR_LENGTH 12

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
        
    // Start the loop from the second element, since the first element is considered sorted in Insertion sort
    for (int index = 1; index < arr.Length; index++)
    {
        // The current item to be placed in the correct position
        int currentItem = arr[index];
        
        // Find the position where the current item should be inserted
        // Start from the element before the current item and move backwards
        int insertionIndex = index - 1;

        // Move elements that are greater than the current item to one position ahead of their current position,
        // to make space for inserting the current item.
        while (insertionIndex >= 0 && arr[insertionIndex] > currentItem)
        {
            arr[insertionIndex + 1] = arr[insertionIndex];
            insertionIndex--;
        }

        // Insert the current item in its correct position.
        arr[insertionIndex + 1] = currentItem;
    }
}
