using Graphs;
using System.Xml.Linq;

namespace BreadthFirstSearch;

/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    BFS traversal sequence: A -> B -> C -> D
*/

public static class GraphExtensions
{
    public static void BFS<T>(this Graph<T> graph, T startValue)
    {
        // Retrieve the start node from the graph using the provided value.
        // If the start node is not found, handle the error appropriately.
        var startNode = graph.GetNode(startValue);
        if (startNode == null)
        {
            // handle
        }

        // Initialize a queue to keep track of nodes
        // For BFS we use Queue - First-In-First-Out (FIFO) data structure
        // queue is used to ensure that nodes are explored in the order they are discovered
        // enabling level-by - level traversal of the graph
        var queue = new Queue<Node<T>>();
        var visited = new HashSet<Node<T>>();

        // Add the start node to the queue and mark it as visited
        queue.Enqueue(startNode);
        visited.Add(startNode);

        // Continue searching as long as there are nodes left to visit
        while (queue.Count > 0)
        {
            var currentNode = queue.Dequeue();

            if (!graph.Edges.TryGetValue(currentNode, out var edges))
                continue;

            foreach (var edge in edges)
            {
                var adjacentNode = edge.Node;

                // If an adjacent node hasn't been visited
                // add it to the queue and mark it as visited
                if (!visited.Contains(adjacentNode))
                {
                    visited.Add(adjacentNode);
                    queue.Enqueue(adjacentNode);
                }
            }
        }
    }
}
