using Graphs;

namespace Prim;

public static class GraphExtensions
{
    public static IEnumerable<NodeEdge<T>> PrimMst<T>(this Graph<T> graph)
    {
        if (graph.Nodes.Count == 0)
        {
            return Enumerable.Empty<NodeEdge<T>>();
        }

        var visited = new HashSet<Node<T>>();

        // PriorityQueue to store nodes along with their edge cost.
        // Nodes are processed based on minimum edge cost.
        var queue = new PriorityQueue<(Node<T>, int), int>();
        var minimumSpanningTree = new List<NodeEdge<T>>();

        // Choose an arbitrary node as the starting point for Prim's algorithm.
        var root = graph.Nodes.First();
        queue.Enqueue((root, 0), 0);

        while (queue.Count > 0)
        {
            // Dequeue the node with the lowest edge cost.
            var queueItem = queue.Dequeue();
            var node = queueItem.Item1;
            var cost = queueItem.Item2;

            // Skip nodes that have already been visited to avoid cycles.
            if (visited.Contains(node))
            {
                continue;
            }

            // Add the node to the MST, along with its cost.
            minimumSpanningTree.Add(new NodeEdge<T>()
            {
                Node = node,
                Cost = cost
            });

            visited.Add(node);

            if(graph.Edges.TryGetValue(node, out var edges))
            {
                foreach (var edge in edges)
                {
                    // For each adjacent node that has not been visited, enqueue it with its cost.
                    // The cost is the key for the priority queue to determine the processing order.
                    if (!visited.Contains(edge.Node))
                    {
                        queue.Enqueue((edge.Node, edge.Cost), edge.Cost);
                    }
                }
            }
        }

        return minimumSpanningTree;
    }

    public class NodeEdge<T>
    {
        public Node<T> Node { get; set; }
        public int Cost { get; set; }
    }
}
