namespace LinkedList;

public class SinglyLinkedList<T>
{
    private class Node
    {
        public T Data;
        public Node Next;

        public Node(T data)
        {
            this.Data = data;
            this.Next = null;
        }
    }

    private Node head;
    private Node tail;
    private int length;

    public int Length
    {
        get { return this.length; }
        private set { length = value; }
    }

    public SinglyLinkedList()
    {
        head = null;
        tail = null;
        length = 0;
    }

    public bool IsEmpty()
    {
        return length == 0;
    }

    public void Add(T data)
    {
        var node = new Node(data);

        if (IsEmpty())
        {
            head = tail = node;
        }
        else
        {
            tail.Next = node;
            tail = node;
        }

        length++;
    }

    public T Get(int index)
    {
        if (IsEmpty() || index < 0 || index >= length)
        {
            throw new IndexOutOfRangeException("Index out of range.");
        }

        var current = head;
        for (int i = 0; i < index; i++)
        {
            current = current.Next;
        }

        return current.Data;
    }

    public void Delete(int index)
    {
        if (IsEmpty() || index < 0 || index >= length)
        {
            throw new IndexOutOfRangeException("Index out of range.");
        }

        if (index == 0)
        {
            head = head.Next;
            if (length == 1)
            {
                tail = null;
            }
        }
        else
        {
            var current = head;
            for (int i = 0; i < index - 1; i++)
            {
                current = current.Next;
            }

            current.Next = current.Next.Next;
            if (index == length - 1) // If the last element is being deleted
            {
                tail = current;
            }
        }

        length--;
    }

    public void Clear()
    {
        head = null;
        tail = null;
        length = 0;
    }
}
