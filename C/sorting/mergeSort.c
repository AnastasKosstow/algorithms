#include <stdio.h>
#include <stdlib.h>
#include <string.h> 

#define ARR_LENGTH 12

void sort(int* sourceArray, int length);

// TODO: Implement!

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };
    sort(arr, ARR_LENGTH);

    return 0;
}

void sort(int* sourceArray, int length)
{
    if (length <= 1) {
        return;
    }

    int middleIndex = length / 2;
    int leftArrayLength = middleIndex;
    int rightArrayLength = length - middleIndex;

    int* leftArray = (int*)malloc(leftArrayLength * sizeof(int));
    int* rightArray = (int*)malloc(rightArrayLength * sizeof(int));

    memcpy(leftArray, sourceArray, leftArrayLength * sizeof(int));
    memcpy(rightArray, sourceArray + middleIndex, rightArrayLength * sizeof(int));

    sort(leftArray, leftArrayLength);
    sort(rightArray, rightArrayLength);

    // merge(sourceArray, leftArray, leftArrayLength, rightArray, rightArrayLength);

    free(leftArray);
    free(rightArray);
}