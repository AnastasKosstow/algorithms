#include <stdio.h>

#define ARR_LENGTH 12
void swap(int* x1, int* x2);

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
    
    for (int index = 0; index < ARR_LENGTH - 1; index++)
    {
        int swapIndex = index;
        while (arr[swapIndex] > arr[swapIndex + 1])
        {
            if (swapIndex < 0) break;

            swap(&arr[swapIndex], &arr[swapIndex + 1]);
            swapIndex--;
        }
    }
}

void swap(int* x1, int* x2)
{
    int temp = *x1;
    *x1 = *x2;
    *x2 = temp;
}
