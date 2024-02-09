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
    internal int Height;
    internal TreeNode<T> Left;
    internal TreeNode<T> Right;

    internal TreeNode(T value)
    {
        Value = value;
        this.Height = 1;
    }

    internal TreeNode<T> GetChildNode(Side side)
    {
        return 
           side switch
           {
               Side.Left => this.Left,
               Side.Right => this.Right,
               _ => throw new ArgumentException(null, nameof(side)),
           };
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
            inserted = InsertValue(Root, value);
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

    private bool InsertValue(TreeNode<T> node, T value)
    {
        if (node.Value.CompareTo(value) == 0)
        {
            return false;
        }

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

        Rebalance(node);
        return true;
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
            Rotate(subtree, side.Switch());
        }
        Rotate(node, side);
    }

    private void Rotate(TreeNode<T> node, Side side)
    {
        var childNode = node.GetChildNode(side);
        if (childNode == null) 
            throw new InvalidOperationException(nameof(childNode));

        if (side == Side.Left)
        {
            var temp = childNode.Right;
            childNode.Right = node;
            node.Left = temp;
        }
        else
        {
            node.Right = childNode;
        }

        if (childNode.GetChildNode(side) != null)
        {
            childNode.GetChildNode(side).UpdateHeight();
        }

        childNode.UpdateHeight();
        node.UpdateHeight();
    }
}
