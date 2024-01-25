namespace UnionFind;

public class UnionFindSet
{
    private int[] parent;

    public UnionFindSet(int size)
    {
        this.parent = Enumerable.Range(0, size).ToArray();
    }
    
    public int Find(int index)
    {
        if (parent[index] != index) 
        {
            parent[index] = this.Find(parent[index]);
        }

        return parent[index];
    }

    public void Union(int x, int y)
    {
        var x_root = this.Find(x);
        var y_root = this.Find(y);
        if (x_root != y_root) 
        {
            this.parent[y_root] = x_root;
        }
    }
}
