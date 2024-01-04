using Graphs;

namespace DepthFirstSearch;

/*  
for graph with nodes: A, B, C, D
    where: A -> B and C; B and C -> D
      A
     / \
    B   C
     \ /
      D

    DFS traversal sequence: A -> C -> D -> B
*/

public static class GraphExtensions
{
    public static void DFS<T>(this Graph<T> graph, T startValue)
    {
        // Retrieve the start node from the graph using the provided value
        // If the start node is not found, handle the error appropriately
        var startNode = graph.GetNode(startValue);
        if (startNode == null)
        {
            // handle
        }

        // Initialize a stack to keep track of nodes
        // For DFS we use Stack - Last-In-First-Out (LIFO) data structure
        // stack is used to explore as far as possible along a branch before backtracking
        // ensuring a depth-first traversal of the graph
        var stack = new Stack<Node<T>>();
        var visited = new HashSet<Node<T>>();

        // Add the start node to the stack and mark it as visited
        stack.Push(startNode);
        visited.Add(startNode);

        // Continue searching as long as there are nodes left to visit
        while (stack.Count > 0)
        {
            var currentNode = stack.Pop();
            if (!graph.Edges.TryGetValue(currentNode, out var edges))
                continue;

            foreach (var edge in edges)
            {
                var adjacentNode = edge.Node;

                // If an adjacent node hasn't been visited
                // add it to the stack and mark it as visited
                if (!visited.Contains(adjacentNode))
                {
                    visited.Add(adjacentNode);
                    stack.Push(adjacentNode);
                }
            }
        }
    }
}
