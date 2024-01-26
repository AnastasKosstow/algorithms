using Graphs;

namespace Kosaraju;

public static class GraphExtensions
{
    public static List<List<Node<T>>> KosarajuSCCs<T>(this Graph<T> graph)
    {
        // Initialize a list to store the strongly connected components.
        var SCCs = new List<List<Node<T>>>();

        if (graph.Nodes.Count == 0)
        {
            return SCCs;
        }

        var reverseGraph = new Graph<T>(graph.GraphType);

        var visited = new HashSet<Node<T>>();
        var stack = new Stack<Node<T>>();

        // Perform the first DFS on the original graph.
        // The purpose of this DFS is to fill the stack with nodes in the order of their finishing times.
        // This is important for the second DFS phase, where nodes are processed in this order to identify SCCs.
        foreach (var node in graph.Nodes)
        {
            if (visited.Contains(node))
            {
                continue;
            }
            Dfs(node, stack, visited);
        }

        Stack<Node<T>> Dfs(Node<T> node, Stack<Node<T>> stack, HashSet<Node<T>> visited)
        {
            if (visited.Contains(node))
            {
                return stack;
            }

            visited.Add(node);

            var hasEdges = graph.Edges.TryGetValue(node, out var edges);
            if (hasEdges && edges.Count > 0)
            {
                foreach (var edge in edges)
                {
                    Dfs(edge.Node, stack, visited);
                }
            }

            stack.Push(node);
            return stack;
        }

        // Construct a mapping of incoming edges for each node.
        // This map will be used in the second DFS, where we need to traverse the graph in the 'reverse' direction without explicitly creating a reversed graph.
        var incomingEdges = new Dictionary<Node<T>, List<Node<T>>>();
        foreach (var node in graph.Nodes)
        {
            if (graph.Edges.TryGetValue(node, out var edges))
            {
                foreach (var edge in edges)
                {
                    if (!incomingEdges.ContainsKey(edge.Node))
                        incomingEdges[edge.Node] = new List<Node<T>>();
                    incomingEdges[edge.Node].Add(node);
                }
            }
        }

        // Reset the visited set for the second DFS.
        visited.Clear();

        // Perform the second DFS using the stack filled during the first DFS. This DFS is done in the reverse
        // order of node completion from the first DFS. It uses the incoming edges map to traverse the graph in reverse.
        // This step is crucial for identifying SCCs, as nodes in an SCC will be reachable from each other in this reverse traversal.
        while (stack.Count != 0)
        {
            var node = stack.Pop();
            if (!visited.Contains(node))
            {
                var component = new List<Node<T>>();
                ReverseDfs(node, component, incomingEdges);

                SCCs.Add(component);
            }
        }

        // The ReverseDfs method recursively explores nodes using the incoming edges map.
        // Nodes are added to the current component if they are reachable in this reversed context.
        // This effectively groups nodes of the same SCC together.
        void ReverseDfs(Node<T> node, List<Node<T>> component, Dictionary<Node<T>, List<Node<T>>> edges)
        {
            if (visited.Contains(node)) return;

            visited.Add(node);
            component.Add(node);

            if (edges.TryGetValue(node, out var predecessors))
            {
                foreach (var pred in predecessors)
                {
                    ReverseDfs(pred, component, edges);
                }
            }
        }

        return SCCs;
    }
}
