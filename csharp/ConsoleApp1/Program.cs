
using Graphs;
using Dijkstra;

var graph = new Graph<string>();
graph.AddNode("A");
graph.AddNode("B");
graph.AddNode("C");
graph.AddNode("D");

graph.AddEdge(graph.GetNode("A"), graph.GetNode("B"), 2);
graph.AddEdge(graph.GetNode("A"), graph.GetNode("C"), 4);
graph.AddEdge(graph.GetNode("B"), graph.GetNode("C"), 1);
graph.AddEdge(graph.GetNode("B"), graph.GetNode("D"), 5);
graph.AddEdge(graph.GetNode("C"), graph.GetNode("D"), 1);

graph.DijkstraShortestPath(graph.GetNode("A"), graph.GetNode("D"));