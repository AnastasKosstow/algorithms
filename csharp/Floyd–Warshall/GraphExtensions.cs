using Graphs;

namespace Floyd_Warshall;

public static class GraphExtensions
{
    public static int FloydWarshallShortestPath<T>(this Graph<T> graph, Node<T> from, Node<T> to)
    {
        var vertices = graph.Nodes.Count;

        // Create a map of each node to an integer index. This mapping is crucial for
        // using a 2D array to represent the distances between each pair of nodes.
        var nodesIndexes = GetNodesIndexes(graph);

        // Initialize a 2D array to store the shortest distances between each pair of vertices.
        // This array will be updated throughout the algorithm to reflect the shortest known distances.
        var distances = new int[vertices, vertices];

        // Initialize the distances array. 
        // The distance from a node to itself is 0; for all other nodes, it is initially set to 'int.MaxValue'.
        // This represents that initially, the shortest distance to these nodes is unknown (infinite).
        for (var row = 0; row < distances.GetLength(0); row++)
        {
            for (var column = 0; column < distances.GetLength(1); column++)
            {
                if (row == column)
                    distances[row, column] = 0;
                else
                    distances[row, column] = int.MaxValue;
            }
        }

        // Populate the distances array with the direct distances between nodes as per the graph's edges.
        // Sets up the base cases for the algorithm, representing the initial known distances.
        foreach (var nodeEdges in graph.Edges)
        {
            var source = nodesIndexes[nodeEdges.Key];
            foreach (var edge in nodeEdges.Value)
            {
                var destination = nodesIndexes[edge.Node];
                distances[source, destination] = edge.Cost;
            }
        }

        // Three nested loops iterate over all triplets of vertices (source, intermediate, destination).
        // We check if the path from source to destination via an intermediate vertex is shorter than the current known path.
        for (int intermediateVertex = 0; intermediateVertex < vertices; intermediateVertex++)
        {
            for (int sourceVertex = 0; sourceVertex < vertices; sourceVertex++)
            {
                for (int destinationVertex = 0; destinationVertex < vertices; destinationVertex++)
                {
                    // Skip if the path from source to intermediate or intermediate to destination is infinite.
                    // This means that there is no path between nodes.
                    if (distances[sourceVertex, intermediateVertex] == int.MaxValue || 
                        distances[intermediateVertex, destinationVertex] == int.MaxValue)
                    {
                        continue;
                    }

                    // Update the distance if a shorter path is found via the intermediate vertex.
                    if (distances[sourceVertex, intermediateVertex] + distances[intermediateVertex, destinationVertex] < distances[sourceVertex, destinationVertex])
                        distances[sourceVertex, destinationVertex] = distances[sourceVertex, intermediateVertex] + distances[intermediateVertex, destinationVertex];
                }
            }
        }

        return distances[nodesIndexes[from], nodesIndexes[to]];
    }

    private static IDictionary<Node<T>, int> GetNodesIndexes<T>(Graph<T> graph)
    {
        var nodesIndex = new Dictionary<Node<T>, int>();
        int index = 0;

        // Creating a node-index mapping is necessary to translate the graph's node references to array indices.
        // This mapping simplifies the algorithm's operations on the distances array.
        foreach (var node in graph.Nodes)
        {
            nodesIndex[node] = index++;
        }
        return nodesIndex;
    }
}
