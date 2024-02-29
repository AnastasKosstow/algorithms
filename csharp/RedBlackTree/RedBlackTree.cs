namespace RedBlackTree;

public enum Side
{
    Left, Right,
}

public enum Color
{
    Red, Black,
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
    public Color Color;
    public TreeNode<T> Left;
    public TreeNode<T> Right;
    public TreeNode<T> Parent;
    public bool IsRoot;

    internal TreeNode(T value, TreeNode<T> parent = null, bool isRoot = false)
    {
        Value = value;
        Parent = parent;
        if (isRoot)
        {
            IsRoot = true;
            Color = Color.Black;
        }
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

    public void ChangeColor()
    {
        this.Color = this.Color switch
        {
            Color.Red => Color.Black,
            Color.Black => Color.Red,
            _ => throw new ArgumentException(null, nameof(Color)),
        };
    }

    public Side? CheckColorMatch()
    {
        if (Left != null && Color == Color.Red && Color == Left.Color)
            return Side.Left;

        if (Right != null && Color == Color.Red && Color == Right.Color)
            return Side.Right;

        return null;
    }
}

public class RedBlackTree<T> where T : IComparable<T>
{
    public TreeNode<T> Root;
    public uint Length { get; private set; }

    public RedBlackTree()
    {
        Root = null;
        Length = 0;
    }

    public bool Insert(T value)
    {
        bool inserted = false;
        if (Root != null)
        {
            inserted = InsertNode(Root, value, inserted);
        }
        else
        {
            Root = new TreeNode<T>(value, isRoot: true);
            inserted = true;
        }

        if (inserted)
        {
            Length++;
        }
        return inserted;
    }

    private bool InsertNode(TreeNode<T> node, T value, bool inserted)
    {
        TreeNode<T> insertedNode = null;
        if (node.Value.CompareTo(value) == 0)
        {
            inserted = false;
            return inserted;
        }

        if (value.CompareTo(node.Value) < 0)
        {
            if (node.Left != null)
                InsertNode(node.Left, value, inserted);
            else
            {
                inserted = true;
                node.Left = new TreeNode<T>(value, parent: node);
                insertedNode = node.Left;
            }
        }
        else
        {
            if (node.Right != null)
                InsertNode(node.Right, value, inserted);
            else
            {
                inserted = true;
                node.Right = new TreeNode<T>(value, parent: node);
                insertedNode = node.Right;
            }
        }

        if (inserted)
        {
            var colorMatch = node.CheckColorMatch();
            if (colorMatch.HasValue)
            {
                CheckForRebalancing(insertedNode, colorMatch.Value);
            }
        }

        return inserted;
    }

    private void CheckForRebalancing(TreeNode<T> node, Side side)
    {
        if (node.Parent == null) return;

        if (node.Color == Color.Red && node.Parent.Color == Color.Red)
        {
            TreeNode<T> grandParent = node.Parent?.Parent;
            TreeNode<T> uncle = null;

            if (grandParent != null)
            {
                if (node.Parent == grandParent.Left)
                    uncle = grandParent.Right;
                else
                    uncle = grandParent.Left;
            }

            if (uncle != null && uncle.Color == Color.Red)
            {
                node.Parent.Color = Color.Black;
                uncle.Color = Color.Black;
                if (grandParent != null)
                {
                    grandParent.Color = grandParent.IsRoot ? Color.Black : Color.Red;
                    CheckForRebalancing(grandParent, side);
                }
            }
            else
            {
                Rotate(side, node, node.Parent);
            }
        }
    }

    private void Rotate(Side side, TreeNode<T> node, TreeNode<T> parent)
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

        node.ChangeColor();
        childNode.ChangeColor();
    }
}
