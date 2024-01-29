namespace UnionFind;

public class UnionFindSet
{
    private int[] parent; // Stores the parent of each element. For a root element, parent[i] = i.
    private int[] rank;   // Stores the approximate depth (or 'rank') of the tree for each root.

    public UnionFindSet(int size)
    {
        this.parent = Enumerable.Range(0, size).ToArray();
        this.rank = new int[size];
    }
    
    public int Find(int index)
    {
        // Path Compression: If 'index' is not the root (parent[index] != index),
        // recursively find the root and update the parent of 'index' for faster future lookups.
        if (parent[index] != index) 
        {
            parent[index] = this.Find(parent[index]);
        }

        return parent[index];
    }

    public void Union(int x, int y)
    {
        // Find the roots of the sets that 'x' and 'y' belong to.
        var x_root = this.Find(x);
        var y_root = this.Find(y);

        // If both elements have the same root, they are already in the same set.
        if (x_root == y_root)
        {
            return;
        }

        // Union by Rank:
        /*
         * When performing a union of two sets, instead of arbitrarily choosing the root for merging, 
         * the root with the smaller rank is attached to the root with the larger rank. 
         * If both roots have the same rank, one is chosen as the new root and its rank is incremented by one.
         * 
         * Attach the smaller tree to the larger tree's root to maintain balanced trees, preventing inefficient linear structures.
         * This approach significantly improves the efficiency of 'Find' operations by keeping tree heights minimal
         */

        if (this.rank[x_root] < this.rank[y_root])
        {
            this.parent[x_root] = y_root;
        }
        else if (this.rank[x_root] > this.rank[y_root])
        {
            this.parent[y_root] = x_root;
        }
        else
        {
            this.parent[y_root] = x_root;
            this.rank[x_root]++;
        }
    }
}
