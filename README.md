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
Iɴ ᴀ ɴᴜᴛꜱʜᴇʟʟ, Bɪɢ-O ɪꜱ ᴀ ʀᴀᴛɪɴɢ ꜰᴏʀ ᴛʜᴇ ᴄᴏᴍᴘʟᴇxɪᴛʏ ᴏꜰ ᴛʜᴇ ᴀʟɢᴏʀɪᴛʜᴍ ʙᴀꜱᴇᴅ ᴏɴ ᴛʜᴇ ᴄᴏꜱᴛ ᴏʀ ᴘʀᴏᴄᴇꜱꜱɪɴɢ ᴛɪᴍᴇ.
<br>
Sᴘᴇᴄɪꜰɪᴄᴀʟʟʏ, ɪᴛ ᴅᴇꜱᴄʀɪʙᴇꜱ ᴛʜᴇ ᴡᴏʀꜱᴛ-ᴄᴀꜱᴇ ꜱᴄᴇɴᴀʀɪᴏ ᴀɴᴅ ᴄᴀɴ ʙᴇ ᴜꜱᴇᴅ ᴛᴏ ᴅᴇꜱᴄʀɪʙᴇ ᴛʜᴇ ᴇxᴇᴄᴜᴛɪᴏɴ ᴛɪᴍᴇ ʀᴇqᴜɪʀᴇᴅ ᴏʀ ᴛʜᴇ ꜱᴘᴀᴄᴇ ᴜꜱᴇᴅ (ᴇ.ɢ., ᴍᴇᴍᴏʀʏ) ʙʏ ᴀɴ ᴀʟɢᴏʀɪᴛʜᴍ.

### Big O key points:
 - <b>𝚃𝚒𝚖𝚎 𝙲𝚘𝚖𝚙𝚕𝚎𝚡𝚒𝚝𝚢:</b> Hᴏᴡ ᴛʜᴇ ᴛɪᴍᴇ ᴛᴏ ᴄᴏᴍᴘʟᴇᴛᴇ ᴛʜᴇ ᴀʟɢᴏʀɪᴛʜᴍ ɪɴᴄʀᴇᴀꜱᴇꜱ ᴀꜱ ᴛʜᴇ ꜱɪᴢᴇ ᴏꜰ ᴛʜᴇ ɪɴᴘᴜᴛ (ɴᴜᴍʙᴇʀ ᴏꜰ ᴇʟᴇᴍᴇɴᴛꜱ ᴛᴏ ʙᴇ ꜱᴏʀᴛᴇᴅ) ɪɴᴄʀᴇᴀꜱᴇꜱ.
 - <b>𝚂𝚙𝚊𝚌𝚎 𝙲𝚘𝚖𝚙𝚕𝚎𝚡𝚒𝚝𝚢:</b> Hᴏᴡ ᴍᴜᴄʜ ᴇxᴛʀᴀ ꜱᴛᴏʀᴀɢᴇ ꜱᴘᴀᴄᴇ ᴛʜᴇ ᴀʟɢᴏʀɪᴛʜᴍ ɴᴇᴇᴅꜱ ʙᴇʏᴏɴᴅ ᴛʜᴇ ᴏʀɪɢɪɴᴀʟ ɪɴᴘᴜᴛ. Tʜɪꜱ ɪꜱ ʟᴇꜱꜱ ᴏꜰᴛᴇɴ ᴅɪꜱᴄᴜꜱꜱᴇᴅ ᴡɪᴛʜ ꜱᴏʀᴛɪɴɢ ᴀʟɢᴏʀɪᴛʜᴍꜱ ʙᴜᴛ ɪꜱ ꜱᴛɪʟʟ ᴀ ʀᴇʟᴇᴠᴀɴᴛ ꜰᴀᴄᴛᴏʀ.

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
      <th width="170">Wᴏʀꜱᴛ ᴄᴀꜱᴇ</th>
      <th width="170">Aᴠᴇʀᴀɢᴇ ᴄᴀꜱᴇ</th>
      <th width="170">Bᴇꜱᴛ ᴄᴀꜱᴇ</th>
      <th width="170">Iᴍᴘʟᴇᴍᴇɴᴛᴀᴛɪᴏɴꜱ</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Bᴜʙʙʟᴇ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/bubble_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BubbleSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Sᴇʟᴇᴄᴛɪᴏɴ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/selection_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/SelectionSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Iɴꜱᴇʀᴛɪᴏɴ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i>)</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/insertion_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/InsertionSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Sʜᴇʟʟ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i><sup>(3/2)</sup>)</td>
      <td>O(<i>n</i><sup>2</sup>)</td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/shell_sort.rs">Rust</a> -
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/ShellSort/Program.cs">C#</a></td>
    </tr>
    <tr>
      <td><b>Mᴇʀɢᴇ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/merge_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/MergeSort/Program.cs">C#</a></td>
    </tr>
  </tbody>
</table>


 
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
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Graphs/Graph.cs">Graph Implementation in C#</a>

### DFS (depth-first search)

- Concept: Understand the <a href="https://en.wikipedia.org/wiki/Depth-first_search">Depth-first search</a>
- Implementations: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dfs.rs">Rust</a> - <a href="">C#</a>

### BFS (breadth-first search)

- Concept: Understand the <a href="https://en.wikipedia.org/wiki/Breadth-first_search">Breadth-first search</a>
- Implementations: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/bfs.rs">Rust</a> - <a href="">C#</a>

### Dijkstra's algorithm
An algorithm for finding the shortest paths between nodes in a weighted graph.

- Concept: Understand the <a href="https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm">Dijkstra's Algorithm</a>
- Implementations: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dijkstra.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Dijkstra/GraphExtensions.cs">C#</a>


LinkedList
==========================

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/list.gif" width="270" alt="list" />

### Key Concepts
 - <b><i>Nodes:</i></b> The individual item or entity in a linked list. Nodes are linked using pointers, making the structure flexible for insertions and deletions.

### Types of LinkedLists
 - <b><i>Singly Linked List:</b></i> Each node has only one pointer to the next node
 - <b><i>Doubly Linked List:</b></i> Each node has two pointers, one to the next node and one to the previous node
 - <b><i>Circular Linked List:</b></i> The last node points back to the first node, forming a circle (it can be singly or doubly)

Implementations (for singly linked list):
<br>
<a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/collections/linked_list.rs">LinkedList Implementation in Rust</a>
<br>
<a href="">LinkedList Implementation in C#</a>
