<div align="center">

<sup>Special thanks to:</sup>

<a href="https://visualgo.net/en">
  <div>
    <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/VisuAlgo.png" width="230" alt="visualgo" />
  </div>
  <b>
    Visit visualgo.net
  </b>
</a>

<hr />
</div>

# Algorithms
This repository contains implementations of many popular algorithms and data structures.

* [Sorting](#sorting)
* [Graphs](#graphs)

*☝ Note that this project is meant to be used for learning and researching purposes only.*
<br />

<h4 align="left">Languages used for implementation:</h4>
<p align="left">
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/rust-logo.png" alt="rust" width="55" height="55"/>
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/csharp-logo.png" alt="csharp" width="55" height="55"/>
</p>

<h4 align="left">For visual representation of the flow of each algorithm use  <a href="https://visualgo.net/en">VisuAlgo.net</a></h4>


# Sorting
<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/sorting.gif" width="230" alt="sorting" />

   > |                              | Worst case | Average case | Best case | Implementations |
   > | ---------------------------- | ---------- | ------------ | --------- | --------------- |
   > | <b>Bubble sort</b>                  | O(<i>n</i><sup>2</sup>)      | O(<i>n</i><sup>2</sup>)         | O(<i>n</i>)               | <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/bubble_sort.rs">Rust</a>, <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BubbleSort/Program.cs">C#</a>    |
   > | <b>Selection sort</b>              | O(<i>n</i><sup>2</sup>)      | O(<i>n</i><sup>2</sup>)          | O(<i>n</i><sup>2</sup>)   | <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/selection_sort.rs">Rust</a>, <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/SelectionSort/Program.cs">C#</a> |
   > | <b>Insertion sort</b>              | O(<i>n</i><sup>2</sup>)      | O(<i>n</i><sup>2</sup>)          | O(<i>n</i>)               | <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/insertion_sort.rs">Rust</a>, <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/InsertionSort/Program.cs">C#</a> |
   > | <b>Shell sort</b>                  | O(<i>n</i><sup>(3/2)</sup>)  | O(<i>n</i><sup>2</sup>)          | O(<i>n</i> log(<i>n</i>)) | <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/shell_sort.rs">Rust</a>, <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/ShellSort/Program.cs">C#</a>     |
   > | <b>Merge sort</b>                  | O(<i>n</i> log(<i>n</i>))    | O(<i>n</i> log(<i>n</i>))        | O(<i>n</i> log(<i>n</i>)) | <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/merge_sort.rs">Rust</a>, <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/MergeSort/Program.cs">C#</a>     |

<br>
 
Graphs
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/graphs.gif" width="270" alt="graphs" />

### Key Concepts
 - <b><i>Nodes (Vertices):</i></b> The individual items or entities in a graph.
 - <b><i>Edges (Links):</i></b> The connections between nodes.

### Types of Graphs
 - <b><i>Undirected Graphs:</b></i> Symmetric relationships
 - <b><i>Directed Graphs:</b></i> Asymmetric relationships, like web links
 - <b><i>Weighted Graphs:</b></i> Graphs with edge weights, useful in routing problems

Implementations:
<br>
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/graph.rs">Graph Implementation in Rust</a>
<br>
<a href="">Graph Implementation in C#</a>

### Dijkstra's algorithm
An algorithm for finding the shortest paths between nodes in a weighted graph.

- Concept: Understand the <a href="https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm">Dijkstra's Algorithm</a>
- Implementations: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dijkstra.rs">Rust</a>, <a href="">C#</a>
