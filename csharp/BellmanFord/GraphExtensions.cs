using Graphs;

namespace BellmanFord;

public static class GraphExtensions
{
    public static int BellmanFordShortestPath<T>(this Graph<T> graph, Node<T> from, Node<T> to)
    {
        // Initialize a dictionary to store the shortest distance from the 'from' node to every other node.
        // Initially, the distance to the 'from' node is 0 and infinity (int.MaxValue) to all other nodes.
        // 'from' node is 0 because we start from this node
        var distances = graph.Nodes.ToDictionary(key => key, value =>
        {
            return value == from ? 0 : int.MaxValue;
        });

        // This flag helps to stop the algorithm early if no updates are made in a full iteration over all nodes.
        bool updated = true;

        // Relax edges up to 'n-1' times (where 'n' is the number of nodes).
        // This is because the shortest path in a graph without cycles will have at most 'n-1' edges.
        for (int node = 0; node < graph.Nodes.Count - 1; node++)
        {
            if (!updated) {
                break;
            }
            updated = false;

            // Iterate over all edges and update distances if a shorter path is found.
            // This process is called 'relaxation'.
            foreach (var node_edges in graph.Edges)
            {
                var currentNode = node_edges.Key;
                var cost = distances[currentNode];

                // Update the distance if the sum of the current node's distance and the edge cost is smaller.
                foreach (var edge in node_edges.Value)
                {
                    if (cost + edge.Cost < distances[edge.Node])
                    {
                        distances[edge.Node] = cost + edge.Cost;
                        updated = true;
                    }
                }
            }
        }

        // Check for negative weight cycles which can be detected if we can relax any edge further.
        // If a further relaxation is possible, it means there's a negative cycle.
        foreach (var node_edges in graph.Edges)
        {
            var currentNode = node_edges.Key;
            var cost = distances[currentNode];

            foreach (var edge in node_edges.Value)
            {
                if (cost + edge.Cost < distances[edge.Node])
                {
                    // If a negative cycle exists, the shortest path problem has no solution.
                    // handle case
                    return 0;
                }
            }
        }

        // After processing all nodes, retrieve the shortest path distance to the 'to' node
        // If this distance is still int.MaxValue, it means there's no path from 'from' node to 'to' node
        distances.TryGetValue(to, out int shortestPath);
        return shortestPath;
    }
}
