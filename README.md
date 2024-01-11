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
* [Graphs](#graphs)
  * [DFS](#DFS (depth-first search))
* [LinkedList](#linkedlist)

*☝ Note that this project is meant to be used for learning and researching purposes only.*
<br />

<h4 align="left">𝙻𝚊𝚗𝚐𝚞𝚊𝚐𝚎𝚜 𝚞𝚜𝚎𝚍 𝚏𝚘𝚛 𝚒𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</h4>
<p align="left">
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/rust-logo.png" alt="rust" width="55" height="55"/>
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/csharp-logo.png" alt="csharp" width="55" height="55"/>
</p>

<h4 align="left">For visual representation of the flow of each algorithm use  <a href="https://visualgo.net/en">𝚅𝚒𝚜𝚞𝙰𝚕𝚐𝚘.𝚗𝚎𝚝</a></h4>


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

> <b>Fᴏʀ ᴍᴏʀᴇ ɪɴꜰᴏʀᴍᴀᴛɪᴏɴ ᴀʙᴏᴜᴛ 'Bɪɢ O ɴᴏᴛᴀᴛɪᴏɴ'</b> - <a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/big-O-notation.html">cooervo.github.io</a>
<a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/index.html">
  <div>
    <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo-algos.svg" width="270" alt="visualgo" />
  </div>
</a>

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

Quicksort
 
Graphs
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/graphs.gif" width="270" alt="graphs" />

### Key Concepts
 - 𝙽𝚘𝚍𝚎𝚜 (𝚅𝚎𝚛𝚝𝚒𝚌𝚎𝚜): *The individual items or entities in a graph*
 - 𝙴𝚍𝚐𝚎𝚜 (𝙻𝚒𝚗𝚔𝚜): *The connections between nodes*

### Types of Graphs
 - 𝚄𝚗𝚍𝚒𝚛𝚎𝚌𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Symmetric relationships*
 - 𝙳𝚒𝚛𝚎𝚌𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Asymmetric relationships, like web links*
 - 𝚆𝚎𝚒𝚐𝚑𝚝𝚎𝚍 𝙶𝚛𝚊𝚙𝚑𝚜: *Graphs with edge weights, useful in routing problems*

𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:
<br>
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
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="">Rust</a> - <a href="">C#</a>

### Floyd Warshall algorithm
Finds shortest path between all node pairs.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm">Floyd Warshall Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="">Rust</a> - <a href="">C#</a>


LinkedList
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/list.gif" width="270" alt="list" />

### Key Concepts
 - 𝙽𝚘𝚍𝚎𝚜: *The individual item or entity in a linked list. Nodes are linked using pointers, making the structure flexible for insertions and deletions.*

### Types of LinkedLists
 - 𝚂𝚒𝚗𝚐𝚕𝚢 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *Each node has only one pointer to the next node*
 - 𝙳𝚘𝚞𝚋𝚕𝚢 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *Each node has two pointers, one to the next node and one to the previous node*
 - 𝙲𝚒𝚛𝚌𝚞𝚕𝚊𝚛 𝙻𝚒𝚗𝚔𝚎𝚍 𝙻𝚒𝚜𝚝: *The last node points back to the first node, forming a circle (it can be singly or doubly)*

𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜 (for singly linked list):
<br>
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/collections/linked_list.rs">LinkedList Implementation in Rust</a>
<br>
<a href="">LinkedList Implementation in C#</a>
