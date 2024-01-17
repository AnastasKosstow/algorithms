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

# 𝙰𝚕𝚐𝚘𝚛𝚒𝚝𝚑𝚖𝚜
𝚃𝚑𝚒𝚜 𝚛𝚎𝚙𝚘𝚜𝚒𝚝𝚘𝚛𝚢 𝚌𝚘𝚗𝚝𝚊𝚒𝚗𝚜 𝚒𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜 𝚘𝚏 𝚖𝚊𝚗𝚢 𝚙𝚘𝚙𝚞𝚕𝚊𝚛 𝚊𝚕𝚐𝚘𝚛𝚒𝚝𝚑𝚖𝚜 𝚊𝚗𝚍 𝚍𝚊𝚝𝚊 𝚜𝚝𝚛𝚞𝚌𝚝𝚞𝚛𝚎𝚜．

* [Sorting](#sorting)
  * [Bubble sort](#sorting-algorithms)
  * [Selection sort](#sorting-algorithms)
  * [Insertions sort](#sorting-algorithms)
  * [Shell sort](#sorting-algorithms)
  * [Merge sort](#sorting-algorithms)
  * [Quicksort](#sorting-algorithms)
* [Graph Theory](#graph-theory)
* [Graph Algorithms](#graph-algorithms)
  * [Graph Implementation](#graph-implementations)
  * [Depth-First Search](#dfs-depth-first-search)
  * [Breadth-First Search](#bfs-breadth-first-search)
  * [Dijkstra's algorithm](#dijkstras-algorithm)
  * [Bellman-Ford algorithm](#bellman-ford-algorithm)
  * [Kruskal algorithm](#kruskal-algorithm)
* [Data Structures](#data-structures)
  * [LinkedList](#linkedlist)
  * [Disjoint-set](#disjoint-set)

<br />

> [!IMPORTANT]  
> Note that this project is meant to be used for learning and researching purposes only. Most optimal implementation for each algorithm or data structure depends on the use case.


<h4 align="left">𝙻𝚊𝚗𝚐𝚞𝚊𝚐𝚎𝚜 𝚞𝚜𝚎𝚍 𝚏𝚘𝚛 𝚒𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</h4>
<p align="left">
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/rust-logo.png" alt="rust" width="55" height="55"/>
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/csharp-logo.png" alt="csharp" width="55" height="55"/>
</p>

<h4 align="left">For visual representation of the flow of each algorithm or data structure use  <a href="https://visualgo.net/en">𝚅𝚒𝚜𝚞𝙰𝚕𝚐𝚘.𝚗𝚎𝚝</a></h4>


# Sorting
<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/sorting.gif" width="230" alt="sorting" />

### Big O Notation
In a nutshell, we use Big O to describe the efficiency of algorithms.
<br>
It represents an upper bound on the time complexity of an algorithm, indicating how the runtime increases with the size of the input. 
<br>
For example, O(N) suggests a linear increase in time with the size of the input, while O(1) indicates constant time regardless of input size

### Key points:
There are three main mathematical notations used to describe the upper, tight, and lower bounds of algorithm complexity
 - Bɪɢ O (O-ɴᴏᴛᴀᴛɪᴏɴ): It describes the upper bound of the time complexity of an algorithm. (worst-case)
 - Bɪɢ Tʜᴇᴛᴀ (Θ-ɴᴏᴛᴀᴛɪᴏɴ): Big Theta provides a tight bound on the time complexity. (average-case)
 - Bɪɢ Oᴍᴇɢᴀ (Ω-ɴᴏᴛᴀᴛɪᴏɴ): Big Omega describes the lower bound of the time complexity of an algorithm. (best-case)
<br>

> [!NOTE]
> <b>Fᴏʀ ᴍᴏʀᴇ ɪɴꜰᴏʀᴍᴀᴛɪᴏɴ ᴀʙᴏᴜᴛ 'Bɪɢ O ɴᴏᴛᴀᴛɪᴏɴ'</b> - <a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/big-O-notation.html">cooervo.github.io</a>
> <a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/index.html">
>  <div>
>    <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo-algos.svg" width="270" alt="visualgo" />
>  </div>
> </a>

#### Sorting Algorithms: 

<table>
  <thead>
    <tr>
      <th width="170"></th>
      <th width="170">wᴏʀꜱᴛ ᴄᴀꜱᴇ</th>
      <th width="170">aᴠᴇʀᴀɢᴇ ᴄᴀꜱᴇ</th>
      <th width="170">bᴇꜱᴛ ᴄᴀꜱᴇ</th>
      <th width="170">Iᴍᴘʟᴇᴍᴇɴᴛᴀᴛɪᴏɴꜱ</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Bᴜʙʙʟᴇ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>Θ(<i>n</i><sup>2</sup>)</td>
      <td>Ω(<i>n</i>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/bubble_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BubbleSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Sᴇʟᴇᴄᴛɪᴏɴ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>Θ(<i>n</i><sup>2</sup>)</td>
      <td>Ω(<i>n</i><sup>2</sup>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/selection_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/SelectionSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Iɴꜱᴇʀᴛɪᴏɴ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>Θ(<i>n</i><sup>2</sup>)</td>
      <td>Ω(<i>n</i>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/insertion_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/InsertionSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Sʜᴇʟʟ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>(3/2)</sup>)</td>
      <td>Θ(<i>n</i><sup>2</sup>)</td>
      <td>Ω(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/shell_sort.rs">Rust</a> -
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/ShellSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Mᴇʀɢᴇ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>Θ(<i>n</i> log(<i>n</i>))</td>
      <td>Ω(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/merge_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/MergeSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Qᴜɪᴄᴋꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>Θ(<i>n</i> log(<i>n</i>))</td>
      <td>Ω(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/quicksort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/QuickSort/Program.cs">C#</a></td>
    </tr>
  </tbody>
</table>

<br>
 
Graph Theory
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/graphds.gif" width="270" alt="graphs" />

<p>Graphs are an abstract idea that represents connections between objects.</p>

> [!NOTE]
> Formal definition:
> An (undirected) *graph* is a collection Ｖ of *vertices*, and a collection Ｅ of *edges* each of which connects a pair of verices.

### Key Concepts
 - 𝚅𝚎𝚛𝚝𝚒𝚌𝚎𝚜 (𝙽𝚘𝚍𝚎𝚜): *The individual items or entities in a graph*
 - 𝙴𝚍𝚐𝚎𝚜 (𝙻𝚒𝚗𝚔𝚜): *The connections between nodes*
 - Loop: *Loops connect a vertex to itself. This means that edge from vertex Ａ points to the same vertex Ａ*

---

### Representing graphs:

#### *Adjacency Matrix*
 - *Vertices Representation*: Each vertex in the graph is associated with one row and one column in the matrix.
   For a graph with ｎ vertices, the adjacency matrix is an ｎ×ｎ square matrix
 - *Edges Representation*:
   - In an undirected graph, if there is an edge between vertex 𝚒 and vertex 𝚓, then the matrix element ![adjacency matrix notation](https://quicklatex.com/cache3/9f/ql_586f6d0f5d7763e7ba1cdd8294598b9f_l3.png) as well ![adjacency matrix notation](https://quicklatex.com/cache3/d9/ql_bc8dbd1d74ceeaed97bfec6bc7ef68d9_l3.png) (since the edge is bidirectional). If there's no edge, ![adjacency matrix notation](https://quicklatex.com/cache3/59/ql_f3e6c2d906774e11eb7cb2b0704bea59_l3.png)
   - In a directed graph ![adjacency matrix notation](https://quicklatex.com/cache3/9f/ql_586f6d0f5d7763e7ba1cdd8294598b9f_l3.png) if there is a directed edge from vertex 𝚒 to vertex 𝚓.  If there's no directed edge from 𝚒 to 𝚓, than ![adjacency matrix notation](https://quicklatex.com/cache3/59/ql_f3e6c2d906774e11eb7cb2b0704bea59_l3.png)
 - *Weights and Multiple Edges*: For weighted graphs, instead of using 1 to indicate an edge, the actual weight of the edge is used. In graphs with multiple edges, the matrix can contain values higher than 1. 

![adjacency matrix notation](https://quicklatex.com/cache3/53/ql_c0572d89a4b47d8ddcea87c68d964c53_l3.png)

#### *Adjacency List*
 - *Vertices Representation*: Graph is an array or a list of lists. Each element of this array (or list) corresponds to a vertex in the graph.
 - *Edges Representation*:
   - For each vertex 𝚒, the adjacency list stores a list of vertices that are adjacent to 𝚓.
   - Implemented using an array of linked lists, an array of arrays, hash table or a map where keys are vertices and values are lists of adjacent vertices.
 - *Directed and Undirected Graphs*:
   - In an undirected graph, if vertex 𝚒 is connected to vertex 𝚓, then 𝚒 will appear in 𝚓's list and 𝚓 will appear in 𝚒's list.
   - In a directed graph, if there is an edge from 𝚒 to 𝚓, then 𝚓 appears in 𝚒's list but not necessarily vice versa.
 - *Weights*: If the graph is weighted, each entry in the list can be a pair (or a structure) containing the adjacent vertex and the weight of the edge. 

<br>

> *degree - The degree of a node in a graph is the number of edges that are connected to it.
<table>
  <thead>
    <tr>
      <th width="170"></th>
      <th width="140">Is edge</th>
      <th width="140">List edge</th>
      <th width="140">List neighbors</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Adjacency Matrix</b></td>
      <td>Θ(<i>1</i>)</td>
      <td>Θ(<i>|V|</i><sup>2</sup>)</td>
      <td>Θ(<i>|V|</i>)</td>
    </tr>
    <tr>
      <td><b>Adjacency List</b></td>
      <td>Θ(<i>degree</i>)</td>
      <td>Θ(<i>|E|</i>)</td>
      <td>Θ(<i>degree</i>)</td>
    </tr>
  </tbody>
</table>

---

### Algorithm runtime
Graph algorithm runtime depents on <b>|V|</b> and <b>|E|</b>
 - <b>|V|</b>: number on vertices
 - <b>|E|</b>: number on edges

Graph performance depends on the *density*.
Graph density is a measure of how many edges are in the graph compared to the maximum possible number of edges between vertices.
 - Dense graph - <b>|E| &asymp; |V|<sup>2</sup></b>
 - Sparse graph - <b>|E| &asymp; |V|</b>

### Types of Graphs
- 𝚄𝚗𝚍𝚒𝚛𝚎𝚌𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Symmetric relationships*
- 𝙳𝚒𝚛𝚎𝚌𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Asymmetric relationships, like web links*
- 𝚆𝚎𝚒𝚐𝚑𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Graphs with edge weights, useful in routing problems*

<br>

Graph Algorithms
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/graphs.gif" width="270" alt="graphs" />

### Graph Implementations:
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/graph.rs">Graph Implementation in Rust</a>
<br>
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Graphs/Graph.cs">Graph Implementation in C#</a>

### DFS (depth-first search)

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Depth-first_search">Depth-first search</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dfs.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/DepthFirstSearch/GraphExtensions.cs">C#</a>

### BFS (breadth-first search)

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Breadth-first_search">Breadth-first search</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/bfs.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BreadthFirstSearch/GraphExtensions.cs">C#</a>

### Dijkstra's algorithm
Finds shortest path froma single source. Does not work with negative weight edges.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm">Dijkstra's Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dijkstra.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Dijkstra/GraphExtensions.cs">C#</a>

### Bellman-Ford algorithm
Finds shortest path from a single source. Works with negative weight edges and reports negative-weight cycles.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm">Bellman-Ford Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/bellman_ford.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BellmanFord/GraphExtensions.cs">C#</a>

### Floyd Warshall algorithm
Finds shortest path between all node pairs.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm">Floyd Warshall Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="">Rust</a> - <a href="">C#</a>

### Kruskal algorithm
Find the minimum spanning tree of a connected, undirected graph.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Kruskal%27s_algorithm">Kruskal Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/kruskal.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Kruskal/GraphExtensions.cs">C#</a>

<br>

Data Structures
==========================

### LinkedList

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/list.gif" width="270" alt="list" />

### Key Concepts
 - 𝙽𝚘𝚍𝚎𝚜: *The individual item or entity in a linked list. Nodes are linked using pointers, making the structure flexible for insertions and deletions.*

### Types of LinkedLists
 - 𝚂𝚒𝚗𝚐𝚕𝚢 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *Each node has only one pointer to the next node*
 - 𝙳𝚘𝚞𝚋𝚕𝚢 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *Each node has two pointers, one to the next node and one to the previous node*
 - 𝙲𝚒𝚛𝚌𝚞𝚕𝚊𝚛 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *The last node points back to the first node, forming a circle (it can be singly or doubly)*

### Implementations (for singly linked list):
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/linked_list.rs">LinkedList Implementation in Rust</a>
<br>
<a href="">LinkedList Implementation in C#</a>

<br>

### Disjoint-set

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/ufds.gif" width="270" alt="list" />

Disjoint-set Data Structure also known as a union-find, keeps track of a set of elements partitioned into several non-overlapping subsets. 

### Key Concepts
It provides two primary operations:
 - Fɪɴᴅ: *Determines which subset a particular element is in. This can be used for determining if two elements are in the same subset*
 - Uɴɪᴏɴ: *Joins two subsets into a single subset*

### Implementations
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/disjoint_set.rs">Disjoint-set(Union-Find) Implementation in Rust</a>
<br>
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/UnionFind/UnionFindSet.cs">Disjoint-set(Union-Find) Implementation in C#</a>

