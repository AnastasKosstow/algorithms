using System.Drawing;
using System.Text.RegularExpressions;

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
    private int Height;
    private TreeNode<T> Left;
    private TreeNode<T> Right;
    private TreeNode<T> Parent;

    public TreeNode(T value)
    {
        Value = value;
        this.Height = 1;
    }

    public TreeNode<T> GetChildNode(Side side)
    {
        return side switch
           {
               Side.Left => this.Left,
               Side.Right => this.Right,
               _ => throw new ArgumentException(null, nameof(side)),
           };
    }

    public int NodeHeight(Side side)
    {
        var node = this.GetChildNode(side);
        return node != null
            ? node.Height
            : 0;
    }

    public void UpdateHeight()
    {
        this.Height = 1 + Math.Max(this.NodeHeight(Side.Left), this.NodeHeight(Side.Right));
    }

    public int GetBalanceFactor()
    {
        var leftHeight = this.NodeHeight(Side.Left);
        var rightHeight = this.NodeHeight(Side.Right);

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

}
