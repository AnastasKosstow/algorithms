using Graphs;
using UnionFind;

namespace Kruskal;

public static class GraphExtensions
{
    public static IEnumerable<NodeEdge<T>> KruskalMst<T>(this Graph<T> graph)
    {
        if (graph.Nodes.Count == 0)
        {
            return Enumerable.Empty<NodeEdge<T>>();
        }

        // Map each node to a unique integer index for use in the union-find data structure
        var nodeIndexMap = new Dictionary<Node<T>, int>();
        int index = 0;
        foreach (var node in graph.Nodes)
        {
            nodeIndexMap[node] = index++;
        }

        // Convert the graph's edges into a flat list and sort them by cost
        // Sorting the edges is a key step in Kruskal's algorithm, which processes edges in ascending order of cost
        var nodeEdges = graph.Edges
            .SelectMany(node => node.Value
                .Select(edge => new NodeEdge<T>()
                {
                    Cost = edge.Cost,
                    From = node.Key,
                    To = edge.Node
                }))
            .OrderBy(nodeEdge => nodeEdge.Cost);

        // The Union-Find structure quickly tells us if two nodes are in the same subset by checking if they have the same root.
        // If they do, it means they are already connected, and adding an edge between them would form a cycle.
        var unionFind = new UnionFindSet(graph.Nodes.Count);

        // Result
        var minimumSpanningTree = new List<NodeEdge<T>>();

        foreach (var nodeEdge in nodeEdges)
        {
            // Determines if the nodes at either end of the current edge (nodeEdge.From and nodeEdge.To) are in different subsets (trees).
            // If they are in different subsets, it means that adding this edge will not create a cycle.
            // If they are in the same subset, adding the edge would complete a cycle, which we must avoid.
            var nodeX = unionFind.Find(nodeIndexMap[nodeEdge.From]);
            var nodeY = unionFind.Find(nodeIndexMap[nodeEdge.To]);

            // Ensures that the current edge connects two different subsets
            // If nodeX and nodeY are different, the nodes are in different subsets, and adding the edge is safe
            // WHY?
            // When it's confirmed that adding the edge won't create a cycle,
            // the Union operation merges the two subsets into a single subset.
            // This step is essential for progressively building the MST, as it connects different parts of the graph.
            if (nodeX != nodeY)
            {
                unionFind.Union(nodeX, nodeY);
                minimumSpanningTree.Add(nodeEdge);
            }
        }

        return minimumSpanningTree;
    }

    public class NodeEdge<T>
    {
        public Node<T> From { get; set; }
        public Node<T> To { get; set; }
        public int Cost { get; set; }
    }
}
