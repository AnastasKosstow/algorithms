namespace BinaryHeap;

public enum HeapType
{
    Min,
    Max
}

public class Heap<T> where T : IComparable<T>
{
    private List<T> heap;
    private HeapType heapType;

    public Heap() : this(HeapType.Max)
    {
    }

    public Heap(HeapType heapType)
    {
        heap = new List<T>();
        this.heapType = heapType;
    }

    public bool IsEmpty()
    {
        return this.heap.Count == 0;
    }

    public void Push(T item)
    {
        var oldLength = heap.Count;
        heap.Add(item);
        SiftUp(oldLength);
    }

    public T Pop()
    {
        if (this.IsEmpty())
        {
            return default;
        }

        var max = heap[0];
        var last = heap[heap.Count - 1];
        heap.RemoveAt(heap.Count - 1);

        if (heap.Count > 0)
        {
            heap[0] = last;
            SiftDown(0);
        }
        return max;
    }

    private void SiftUp(int index)
    {
        if (index == 0)
        {
            return;
        }

        int parentIndex = (index - 1) / 2;
        Func<T, T, bool> shouldSwap = (heapType == HeapType.Max) ?
            new Func<T, T, bool>((a, b) => a.CompareTo(b) > 0) :
            new Func<T, T, bool>((a, b) => a.CompareTo(b) < 0);

        if (shouldSwap(heap[index], heap[parentIndex]))
        {
            (heap[index], heap[parentIndex]) = (heap[parentIndex], heap[index]);
            SiftUp(parentIndex);
        }
    }

    private void SiftDown(int index)
    {
        int leftChildIndex = index * 2 + 1;
        int rightChildIndex = index * 2 + 2;

        if (leftChildIndex >= heap.Count)
        {
            return;
        }

        Func<T, T, bool> comparison = (heapType == HeapType.Max) ?
            new Func<T, T, bool>((a, b) => a.CompareTo(b) > 0) :
            new Func<T, T, bool>((a, b) => a.CompareTo(b) < 0);

        int childIndex = leftChildIndex;
        if (rightChildIndex < heap.Count && comparison(heap[rightChildIndex], heap[leftChildIndex]))
        {
            childIndex = rightChildIndex;
        }

        if (comparison(heap[childIndex], heap[index]))
        {
            (heap[index], heap[childIndex]) = (heap[childIndex], heap[index]);
            SiftDown(childIndex);
        }
    }
}
