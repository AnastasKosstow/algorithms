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

    internal void ChangeColor()
    {
        this.Color = this.Color switch
        {
            Color.Red => Color.Black,
            Color.Black => Color.Red,
            _ => throw new ArgumentException(null, nameof(Color)),
        };
    }

    internal Side? CheckColorMatch()
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
                inserted = InsertNode(node.Left, value, inserted);
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
                inserted = InsertNode(node.Right, value, inserted);
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
                if (uncle == null || uncle.Color == Color.Black)
                {
                    if (grandParent != null)
                    {
                        if (node.Parent != grandParent.GetChildNode(side))
                        {
                            Rotate(side, node.Parent);
                            side = side.Switch();
                        }

                        Rotate(side, grandParent);
                        node.ChangeColor();
                        node.GetChildNode(side.Switch()).ChangeColor();
                    }
                }

                Root.Color = Color.Black; // Ensure root is always black
            }
        }
    }

    private void Rotate(Side side, TreeNode<T> node)
    {
        // Save child and grandchild nodes
        TreeNode<T> child = node.GetChildNode(side);
        TreeNode<T> grandChild = child?.GetChildNode(side.Switch());

        // Perform rotation
        if (node.IsRoot)
        {
            Root = child;
            child.Parent = null;
            child.IsRoot = true;
        }
        else
        {
            TreeNode<T> parent = node.Parent;
            if (parent.Left == node)
                parent.Left = child;
            else
                parent.Right = child;

            child.Parent = parent;
        }

        node.Parent = child;
        if (side == Side.Left)
        {
            child.Right = node;
            node.Left = grandChild;
        }
        else
        {
            child.Left = node;
            node.Right = grandChild;
        }

        // Reassign grandchild
        if (grandChild != null)
            grandChild.Parent = node;
    }
}
