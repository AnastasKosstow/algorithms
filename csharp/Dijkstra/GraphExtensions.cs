using Graphs;

namespace Dijkstra;

public static class GraphExtensions
{
    public static int DijkstraShortestPath<T>(this Graph<T> graph, Node<T> from, Node<T> to)
    {
        // Initialize a dictionary to store the shortest distance from the 'from' node to every other node.
        // Initially, the distance to the 'from' node is 0 and infinity (int.MaxValue) to all other nodes.
        // 'from' node is 0 because we start from this node
        var distances = graph.Nodes.ToDictionary(key => key, value =>
        {
            return value == from ? 0 : int.MaxValue;
        });

        // HashSet to keep track of visited nodes to avoid reprocessing.
        var visited = new HashSet<Node<T>>();

        // PriorityQueue to select the node with the minimum distance for processing next.
        // It stores pairs of (Node, Distance) and sorts them by Distance.
        var queue = new PriorityQueue<(Node<T>, int), int>();
        queue.Enqueue((from, 0), 0);

        while (queue.Count > 0)
        {
            // Dequeue the node with the smallest known distance.
            // On first step this is 'from' node with 0
            var dequeueItem = queue.Dequeue();
            var node = dequeueItem.Item1;
            var cost = dequeueItem.Item2;

            // Check if the current node has adjacent nodes.
            if (graph.Edges.TryGetValue(node, out var edges))
            {
                // For each adjacent node, calculate the distance through the current node.
                foreach (var edge in edges)
                {
                    // This line checks if the current path to the adjacent node (edge.Node) is shorter than any previously found paths
                    // The 'distances.TryGetValue(edge.Node, out int value)' attempts to get the current shortest known distance to the adjacent node
                    // The 'cost > value' part checks if the current total cost to reach 'edge.Node'
                    // via the current node is greater than this known shortest distance ('value')
                    // If it is, this means that the current path is not shorter than the already known path to 'edge.Node',and we skip
                    if (!distances.TryGetValue(edge.Node, out int value) || cost > value)
                        continue;

                    // If the path through the current node is shorter, update the distance of the adjacent node.
                    if ((cost + edge.Cost) < distances[edge.Node])
                    {
                        var newCost = cost + edge.Cost;
                        distances[edge.Node] = newCost;

                        // If the adjacent node is unvisited, add it to the queue for further processing.
                        if (!visited.Contains(edge.Node))
                        {
                            queue.Enqueue((edge.Node, newCost), newCost);
                        }
                    }
                }
                // Mark the current node as visited
                visited.Add(node);
            }
        }

        // After processing all nodes, retrieve the shortest path distance to the 'to' node
        // If this distance is still int.MaxValue, it means there's no path from 'from' node to 'to' node
        distances.TryGetValue(to, out int shortestPath);
        if (shortestPath == int.MaxValue)
        {
            // handle the case where no path exists
        }
        return shortestPath;
    }
}
