namespace AVLTree;

public enum Side
{
    Left, Right,
}

public static class SideExtensions
{
    public static Side Switch(this Side side)
        => side switch
        {
            Side.Left => Side.Right,
            Side.Right => Side.Left,
            _ => throw new ArgumentOutOfRangeException(nameof(side), side, null),
        };
}

public class TreeNode<T> where T : IComparable<T>
{
    public T Value;
    public int Height;
    public TreeNode<T> Left;
    public TreeNode<T> Right;

    internal TreeNode(T value)
    {
        Value = value;
        this.Height = 1;
    }

    internal TreeNode<T> GetChildNode(Side side)
    {
        var node = side switch
        {
            Side.Left => this.Left,
            Side.Right => this.Right,
            _ => throw new ArgumentException(null, nameof(side)),
        };

        return node;
    }

    internal int GetHeight(Side side)
    {
        var node = this.GetChildNode(side);
        return node != null
            ? node.Height
            : 0;
    }

    internal void UpdateHeight()
    {
        this.Height = 1 + Math.Max(this.GetHeight(Side.Left), this.GetHeight(Side.Right));
    }

    internal int GetBalanceFactor()
    {
        var leftHeight = this.GetHeight(Side.Left);
        var rightHeight = this.GetHeight(Side.Right);

        if (leftHeight > rightHeight)
        {
            return leftHeight - rightHeight;
        }
        else
        {
            return -(rightHeight - leftHeight);
        }
    }
}

public class AVLTree<T> where T : IComparable<T>
{
    public TreeNode<T> Root;
    public uint Length { get; private set; }

    public AVLTree()
    {
        Root = null;
        Length = 0;
    }

    public bool Insert(T value)
    {
        bool inserted = true;
        if (Root != null)
        {
            inserted = InsertNode(Root, value);
        }
        else
        {
            Root = new TreeNode<T>(value);
        }

        if (inserted)
        {
            Length++;
        }
        return inserted;
    }

    public bool Delete(T value)
    {
        bool deleted = false;
        var deletedNode = DeleteNode(Root, value);
        if (deletedNode != null)
        {
            Length--;
            deleted = true;
        }

        return deleted;
    }

    private bool InsertNode(TreeNode<T> node, T value)
    {
        if (node.Value.CompareTo(value) == 0)
        {
            return false;
        }

        if (value.CompareTo(node.Value) < 0)
        {
            if (node.Left != null)
                InsertNode(node.Left, value);
            else
                node.Left = new TreeNode<T>(value);
        }
        else
        {
            if (node.Right != null)
                InsertNode(node.Right, value);
            else
                node.Right = new TreeNode<T>(value);
        }

        Rebalance(node);
        return true;
    }

    private TreeNode<T> DeleteNode(TreeNode<T> node, T value)
    {
        if (node == null)
            return null;

        int compare = value.CompareTo(node.Value);
        if (compare < 0)
        {
            node.Left = DeleteNode(node.Left, value);
        }
        else if (compare > 0)
        {
            node.Right = DeleteNode(node.Right, value);
        }
        else
        {
            if (node.Left == null)
            {
                return node.Right;
            }
            else if (node.Right == null)
            {
                return node.Left;
            }

            TreeNode<T> minNode = FindMin(node.Right);
            node.Value = minNode.Value;
            node.Right = DeleteNode(node.Right, minNode.Value);
        }

        Rebalance(node);
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

    private void Rebalance(TreeNode<T> node)
    {
        node.UpdateHeight();

        Side side;
        var balanceFactor = node.GetBalanceFactor();
        if (balanceFactor == 2)
        {
            side = Side.Left;
        }
        else if (balanceFactor == -2)
        {
            side = Side.Right;
        }
        else
        {
            return;
        }


        var subtree = node.GetChildNode(side);
        var subtreeBalanceFactor = subtree.GetBalanceFactor();
        if ((side == Side.Left && subtreeBalanceFactor == -1) || (side == Side.Right && subtreeBalanceFactor == 1))
        {
            Rotate(side.Switch(), subtree, node);
        }
        Rotate(side, node);
    }

    private void Rotate(Side side, TreeNode<T> node, TreeNode<T> parent = null)
    {
        var childNode = node.GetChildNode(side);
        if (childNode == null)
            throw new InvalidOperationException(nameof(childNode));

        if (side == Side.Left)
        {
            var rightChildNode = childNode.Right;
            childNode.Right = node;
            node.Left = rightChildNode;
        }
        else
        {
            var leftChildNode = childNode.Left;
            childNode.Left = node;
            node.Right = leftChildNode;
        }

        if (parent != null)
        {
            if (side.Switch() == Side.Left)
            {
                parent.Left = childNode;
            }
            else
            {
                parent.Right = childNode;
            }
        }

        if (this.Root.Value.CompareTo(node.Value) == 0)
        {
            this.Root = childNode;
        }

        node.UpdateHeight();
        childNode.UpdateHeight();
    }
}
