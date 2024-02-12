namespace BinarySearchTree;

public class TreeNode<T> where T : IComparable<T>
{
    public T Value;
    public TreeNode<T> Left;
    public TreeNode<T> Right;

    public TreeNode(T value)
    {
        Value = value;
        Left = null;
        Right = null;
    }
}

public class BinarySearchTree<T> where T : IComparable<T>
{
    public TreeNode<T> Root;
    public uint Length { get; private set; }

    public BinarySearchTree()
    {
        Root = null;
        Length = 0;
    }

    public void Insert(T value)
    {
        if (Root != null)
        {
            InsertValue(Root, value);
        }
        else
        {
            Root = new TreeNode<T>(value);
        }
        Length++;
    }

    private void InsertValue(TreeNode<T> node, T value)
    {
        if (value.CompareTo(node.Value) < 0)
        {
            if (node.Left != null)
                InsertValue(node.Left, value);
            else
                node.Left = new TreeNode<T>(value);
        }
        else
        {
            if (node.Right != null)
                InsertValue(node.Right, value);
            else
                node.Right = new TreeNode<T>(value);
        }
    }

    public void Delete(T value)
    {
        Root = DeleteNode(Root, value);
        Length--;
    }

    private TreeNode<T> DeleteNode(TreeNode<T> node, T value)
    {
        if (node == null) 
            return null;

        int compare = value.CompareTo(node.Value);
        if (compare < 0)
            node.Left = DeleteNode(node.Left, value);
        else if (compare > 0)
            node.Right = DeleteNode(node.Right, value);
        else
        {
            if (node.Left == null)
                return node.Right;
            else if (node.Right == null)
                return node.Left;

            TreeNode<T> minNode = FindMin(node.Right);
            node.Value = minNode.Value;
            node.Right = DeleteNode(node.Right, minNode.Value);
        }
        return node;
    }

    private TreeNode<T> FindMin(TreeNode<T> node)
    {
        TreeNode<T> current = node;
        while (current.Left != null)
        {
            current = current.Left;
        }
        return current;
    }

    public List<T> GetAll()
    {
        var all = new List<T>();
        InOrderTraversal(Root, all);
        return all;
    }

    private void InOrderTraversal(TreeNode<T> node, List<T> all)
    {
        if (node == null) return;

        InOrderTraversal(node.Left, all);
        all.Add(node.Value);
        InOrderTraversal(node.Right, all);
    }

    public void Clear()
    {
        Root = null;
        Length = 0;
    }
}
