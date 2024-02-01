using BinaryHeap;

int[] arr = new[] { 1, 7, 3, 2, 9, 5, 4, 2, 7, 3, 8, 6 };

// Create a min heap. A min heap is used here because we want to sort the array in ascending order.
// In a min heap, the smallest element is always at the root. Therefore, by continuously removing the root, we can obtain the elements in ascending order.
// We can use already implemented heap structure from: DataStructures -> BinaryHeap
var heap = new Heap<int>(HeapType.Min);

// Build the heap.
// Insert all elements of the array into the heap.
// The heap will rearrange these elements to maintain the min heap property, where the parent is smaller than its children.
foreach (int item in arr)
{
    heap.Push(item);
}

// Extract elements from the heap one by one and store them back into the array.
// Since this is a min heap, each time we call Pop(), we get the smallest remaining element.
// By doing this for each element, we are effectively sort the array.
for (int index = 0; index < arr.Length; index++)
{
    arr[index] = heap.Pop();
}
