#include <math.h>
#include <stdio.h>

#define ARR_LENGTH 40

void generateKnuthSequence(int sequence[], int arrLength);

int main()
{
    int arr[ARR_LENGTH] = { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6, 8, 4, 3, 5 };        
    int sequence[100]; // Assuming the sequence will not have more than 100 elements
    generateKnuthSequence(sequence, ARR_LENGTH);

    // Iterate over the Knuth sequence
    // and perform insertion sort for every step
    for (int stepIndex = 0; sequence[stepIndex] != 0; stepIndex++) {
        int step = sequence[stepIndex];

        for (int index = step; index < ARR_LENGTH; index++) {
            int currentItem = arr[index];
            int compareIndex = index - step;

            // Shift elements that are step distance apart and greater than currentItem
            while (compareIndex >= 0 && currentItem < arr[compareIndex]) {
                arr[compareIndex + step] = arr[compareIndex];
                compareIndex -= step;
            }

            // Place currentItem in its correct position
            arr[compareIndex + step] = currentItem;
        }
    }
}

// Generate the Knuth sequence based on the array size
// 1, 4, 13, 40, 121, ...
void generateKnuthSequence(int sequence[], int arrLength) {
    int gapSequenceLength = (int)floor(log(arrLength) / log(3)) - 1;
    int value = 1;

    for (int index = 0; index < gapSequenceLength; index++) {
        sequence[index] = value;
        value = 3 * value + 1; // Calculate the next gap size
    }

    // Reverse the sequence as per Knuth's suggestion
    for (int i = 0; i < gapSequenceLength / 2; i++) {
        int temp = sequence[i];
        sequence[i] = sequence[gapSequenceLength - i - 1];
        sequence[gapSequenceLength - i - 1] = temp;
    }

    // Marking the end of the sequence with 0
    sequence[gapSequenceLength] = 0;
}
