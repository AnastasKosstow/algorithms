namespace Graphs;

public enum GraphType 
{
    Directed,
    Undirected
}

public class Node<T>
{
    public T Value { get; }

    public Node(T value) => Value = value;
}

public class Edge<T>
{
    public int Cost { get; }
    public Node<T> Node { get; }

    public Edge(int cost, Node<T> node)
    {
        Cost = cost;
        Node = node;
    }
}

public class Graph<T>
{
    public ICollection<Node<T>> Nodes { get; }
    public IDictionary<Node<T>, ICollection<Edge<T>>> Edges { get; }
    public GraphType GraphType { get; }

    public Graph() : this(GraphType.Undirected)
    {
    }

    public Graph(GraphType graphType)
    {
        Nodes = new List<Node<T>>();
        Edges = new Dictionary<Node<T>, ICollection<Edge<T>>>();
        GraphType = graphType;
    }

    public void AddNode(T value)
    {
        var node = new Node<T>(value);
        this.Nodes.Add(node);
    }

    public void AddEdge(Node<T> fromNode, Node<T> toNode, int cost)
    {
        if (fromNode == null || toNode == null)
        {
            // handle
            return;
        }

        Add(fromNode, toNode, cost);

        if (this.GraphType == GraphType.Undirected)
        {
            Add(toNode, fromNode, cost);
        }

        void Add(Node<T> from, Node<T> to, int cost)
        {
            if (!Edges.TryGetValue(from, out ICollection<Edge<T>> value))
            {
                value = new List<Edge<T>>();
                this.Edges.Add(from, value);
            }

            var edge = new Edge<T>(cost, to);
            value.Add(edge);
        }
    }

    public Node<T> GetNode(T value)
    {
        var node = this.Nodes.FirstOrDefault(x => x.Value.Equals(value));
        if (node.Value == null)
        {
            // handle
        }

        return node;
    }
}
