using Graphs;

namespace Dijkstra;

public static class GraphExtensions
{
    public static int ShortestPath<T>(this Graph<T> graph, Node<T> from, Node<T> to)
    {
        var distances = graph.Nodes.ToDictionary(key => key, value =>
        {
            return value == from 
                ? 0 
                : int.MaxValue;
        });
        var visited = new HashSet<Node<T>>();

        var queue = new PriorityQueue<(Node<T>, int), int>();
        queue.Enqueue((from, 0), 0);

        while (queue.Count > 0) 
        {
            var dequeueItem = queue.Dequeue();
            var node = dequeueItem.Item1;
            var cost = dequeueItem.Item2;

            if (graph.Edges.TryGetValue(node, out var edges))
            {
                foreach (var edge in edges)
                {
                    if (!distances.TryGetValue(edge.Node, out int value) || cost > value)
                    {
                        continue;
                    }

                    if ((cost + edge.Cost) < distances[edge.Node])
                    {
                        var newCost = cost + edge.Cost;
                        distances[edge.Node] = cost + edge.Cost;
                        if (!visited.Contains(edge.Node))
                        {
                            queue.Enqueue((edge.Node, newCost), newCost);
                        }
                    }
                }
                visited.Add(node);
            }
        }

        distances.TryGetValue(to, out int shortestPath);
        if (shortestPath == 0)
        {
            // handle
        }
        return shortestPath;
    }
}
