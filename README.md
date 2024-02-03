<div>

# 𝚁𝚎𝚜𝚘𝚞𝚛𝚌𝚎𝚜

<table>
  <tr align="center">
    <td>
      <a href="https://visualgo.net/en">
        <div>
          <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/VisuAlgo.png" width="250" alt="visualgo" />
        </div>
      </a>
    </td>
    <td>
      <a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/big-O-notation.html">
        <div>
          <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo-algos.svg" width="230" alt="visualgo" />
        </div>
      </a>
    </td>
    <td>
      <a href="https://algorithms.discrete.ma.tum.de">
        <div>
          <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/TUM.png" width="230" alt="visualgo" />
        </div>
      </a>
    </td>
  </tr>
  <tr>
    <td><b><a href="https://visualgo.net/en">ᴠɪꜱᴜᴀʟɢᴏ.ɴᴇᴛ</a></td>
    <td><b><a href="https://cooervo.github.io/Algorithms-DataStructures-BigONotation/big-O-notation.html">ᴄᴏᴏᴇʀᴠᴏ.ɢɪᴛʜᴜʙ.ɪᴏ</a></td>
    <td><b><a href="https://algorithms.discrete.ma.tum.de">ᴀʟɢᴏʀɪᴛʜᴍꜱ.ᴅɪꜱᴄʀᴇᴛᴇ.ᴍᴀ.ᴛᴜᴍ.ᴅᴇ</a></td>
  </tr>
</table>

<hr />
</div>

# 𝙰𝚕𝚐𝚘𝚛𝚒𝚝𝚑𝚖𝚜 & 𝙳𝚊𝚝𝚊 𝚂𝚝𝚛𝚞𝚌𝚝𝚞𝚛𝚎𝚜
𝚃𝚑𝚒𝚜 𝚛𝚎𝚙𝚘𝚜𝚒𝚝𝚘𝚛𝚢 𝚌𝚘𝚗𝚝𝚊𝚒𝚗𝚜 𝚒𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜 𝚘𝚏 𝚖𝚊𝚗𝚢 𝚙𝚘𝚙𝚞𝚕𝚊𝚛 𝚊𝚕𝚐𝚘𝚛𝚒𝚝𝚑𝚖𝚜 𝚊𝚗𝚍 𝚍𝚊𝚝𝚊 𝚜𝚝𝚛𝚞𝚌𝚝𝚞𝚛𝚎𝚜．

* [Sorting](#sorting)
  * [Bubble sort](#sorting-algorithms)
  * [Selection sort](#sorting-algorithms)
  * [Insertions sort](#sorting-algorithms)
  * [Shell sort](#sorting-algorithms)
  * [Heap sort](#sorting-algorithms)
  * [Merge sort](#sorting-algorithms)
  * [Quicksort](#sorting-algorithms)
* [Data Structures](#data-structures)
  * [Trees](#tree-theory)
    * [Tree Theory](#tree-theory)
    * [Binary Heap](#heap) 
    * [Binary Search Tree](#binary-search-tree) 
  * [LinkedList](#linkedlist)
  * [Disjoint-set](#disjoint-set)
* [Graph Theory](#graph-theory)
* [Graph Implementation](#graph-implementations)
* [Graph Algorithms](#graph-algorithms)
  * [Traversal](#dfs-depth-first-search)
    * [Depth-First Search](#dfs-depth-first-search)
    * [Breadth-First Search](#bfs-breadth-first-search)
  * [Shortest path](#dijkstras-algorithm)
    * [Dijkstra](#dijkstras-algorithm)
    * [Bellman-Ford](#bellman-ford-algorithm)
    * [Floyd-Warshall](#floyd-warshall-algorithm)
  * [Spanning tree](#kruskal-algorithm)
    * [Kruskal](#kruskal-algorithm)
    * [Prim](#prims-algorithm)
  * [Connected components](#Kosarajus-algorithm)
    * [Kosaraju](#Kosarajus-algorithm)
    * [Tarjan](#tarjans-algorithm)

<br />

> [!IMPORTANT]  
> Note that this project is meant to be used for learning and researching purposes only. Most optimal implementation for each algorithm or data structure depends on the use case.


<h4 align="left">𝙻𝚊𝚗𝚐𝚞𝚊𝚐𝚎𝚜 𝚞𝚜𝚎𝚍 𝚏𝚘𝚛 𝚒𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</h4>
<p align="left">
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/rust-logo.png" alt="rust" width="55" height="55"/>
  <img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/logo/csharp-logo.png" alt="csharp" width="55" height="55"/>
</p>

<h4 align="left">For visual representation of the flow of each algorithm or data structure use  <a href="https://visualgo.net/en">𝚅𝚒𝚜𝚞𝙰𝚕𝚐𝚘.𝚗𝚎𝚝</a> & <a href="https://algorithms.discrete.ma.tum.de">ᴀʟɢᴏʀɪᴛʜᴍꜱ.ᴅɪꜱᴄʀᴇᴛᴇ.ᴍᴀ.ᴛᴜᴍ.ᴅᴇ</a></h4>


# Sorting
<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/sorting.gif" width="270" alt="sorting" />

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
      <td><b>Hᴇᴀᴘ ꜱᴏʀᴛ</b></td>
      <td>O(<i>n</i> log(<i>n</i>))</td>
      <td>Θ(<i>n</i> log(<i>n</i>))</td>
      <td>Ω(<i>n</i> log(<i>n</i>))</td>
      <td>
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/sorting/heap_sort.rs">Rust</a> - 
        <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/HeapSort/Program.cs">C#</a></td>
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

Data Structures
==========================

### Tree Theory

𝖠 𝗍𝗋𝖾𝖾 𝗂𝗌 𝖺 𝖽𝖺𝗍𝖺 𝗌𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾 𝖼𝗈𝗆𝗉𝗈𝗌𝖾𝖽 𝗈𝖿 𝗇𝗈𝖽𝖾𝗌.
 * 𝖤𝖺𝖼𝗁 𝗍𝗋𝖾𝖾 𝗁𝖺𝗌 𝖺 𝗋𝗈𝗈𝗍 𝗇𝗈𝖽𝖾.
 * 𝖳𝗁𝖾 𝗋𝗈𝗈𝗍 𝗇𝗈𝖽𝖾 𝗁𝖺𝗌 𝗓𝖾𝗋𝗈 𝗈𝗋 𝗆𝗈𝗋𝖾 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾𝗌.
 * 𝖤𝖺𝖼𝗁 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 𝗁𝖺𝗌 𝗓𝖾𝗋𝗈 𝗈𝗋 𝗆𝗈𝗋𝖾 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾𝗌, 𝖺𝗇𝖽 𝗌𝗈 𝗈𝗇.

𝖳𝗁𝖾 𝗍𝗋𝖾𝖾 𝖼𝖺𝗇𝗇𝗈𝗍 𝖼𝗈𝗇𝗍𝖺𝗂𝗇 𝖼𝗒𝖼𝗅𝖾𝗌. 𝖳𝗁𝖾 𝗇𝗈𝖽𝖾𝗌 𝗆𝖺𝗒 𝗈𝗋 𝗆𝖺𝗒 𝗇𝗈𝗍 𝖻𝖾 𝗂𝗇 𝖺 𝗉𝖺𝗋𝗍𝗂𝖼𝗎𝗅𝖺𝗋 𝗈𝗋𝖽𝖾𝗋 𝖺𝗇𝖽 𝗍𝗁𝖾𝗒 𝖼𝗈𝗎𝗅𝖽 𝗁𝖺𝗏𝖾 𝖺𝗇𝗒 𝖽𝖺𝗍𝖺
𝗍𝗒𝗉𝖾 𝖺𝗌 𝗏𝖺𝗅𝗎𝖾𝗌.

#### Binary Tree vs. Binary Search Tree
𝖠 `𝖻𝗂𝗇𝖺𝗋𝗒 𝗌𝖾𝖺𝗋𝖼𝗁 𝗍𝗋𝖾𝖾` 𝗂𝗌 𝖺 𝖻𝗂𝗇𝖺𝗋𝗒 𝗍𝗋𝖾𝖾 𝗂𝗇 𝗐𝗁𝗂𝖼𝗁 𝖾𝗏𝖾𝗋𝗒 𝗇𝗈𝖽𝖾 𝖿𝗂𝗍𝗌 𝖺 𝗌𝗉𝖾𝖼𝗂𝖿𝗂𝖼 𝗈𝗋𝖽𝖾𝗋𝗂𝗇𝗀 𝗉𝗋𝗈𝗉𝖾𝗋𝗍𝗒.
𝖳𝗁𝗂𝗌 𝗆𝗎𝗌𝗍 𝖻𝖾 𝗍𝗋𝗎𝖾 𝖿𝗈𝗋 𝖾𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 *𝗇*.

> 𝖡𝗂𝗇𝖺𝗋𝗒 𝖲𝖾𝖺𝗋𝖼𝗁 𝖯𝗋𝗈𝗉𝖾𝗋𝗍𝗒:
> - 𝖥𝗈𝗋 𝖾𝗏𝖾𝗋𝗒 𝗇𝗈𝖽𝖾, 𝖺𝗅𝗅 𝖾𝗅𝖾𝗆𝖾𝗇𝗍𝗌 𝗂𝗇 𝗍𝗁𝖾 𝗅𝖾𝖿𝗍 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝖺𝗋𝖾 𝗅𝖾𝗌𝗌 𝗍𝗁𝖺𝗇 𝗍𝗁𝖾 𝗇𝗈𝖽𝖾'𝗌 𝗏𝖺𝗅𝗎𝖾, 𝖺𝗇𝖽 𝖺𝗅𝗅 𝖾𝗅𝖾𝗆𝖾𝗇𝗍𝗌 𝗂𝗇 𝗍𝗁𝖾 𝗋𝗂𝗀𝗁𝗍 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝖺𝗋𝖾 𝗀𝗋𝖾𝖺𝗍𝖾𝗋 𝗍𝗁𝖺𝗇 𝗍𝗁𝖾 𝗇𝗈𝖽𝖾'𝗌 𝗏𝖺𝗅𝗎𝖾.

𝖳𝗁𝗂𝗌 𝗂𝗇𝖾𝗊𝗎𝖺𝗅𝗂𝗍𝗒 𝗆𝗎𝗌𝗍 𝖻𝖾 𝗍𝗋𝗎𝖾 𝖿𝗈𝗋 𝖺𝗅𝗅 𝗈𝖿 𝖺 𝗇𝗈𝖽𝖾'𝗌 𝖽𝖾𝗌𝖼𝖾𝗇𝖽𝖾𝗇𝗍𝗌, 𝗇𝗈𝗍 𝗃𝗎𝗌𝗍 𝗂𝗍𝗌 𝗂𝗆𝗆𝖾𝖽𝗂𝖺𝗍𝖾 𝖼𝗁𝗂𝗅𝖽𝗋𝖾𝗇. 𝖳𝗁𝖾
𝖿𝗈𝗅𝗅𝗈𝗐𝗂𝗇𝗀 𝗍𝗋𝖾𝖾 𝗈𝗇 𝗍𝗁𝖾 𝗅𝖾𝖿𝗍 𝖻𝖾𝗅𝗈𝗐 𝗂𝗌 𝖺 𝖻𝗂𝗇𝖺𝗋𝗒 𝗌𝖾𝖺𝗋𝖼𝗁 𝗍𝗋𝖾𝖾. 𝖳𝗁𝖾 𝗍𝗋𝖾𝖾 𝗈𝗇 𝗍𝗁𝖾 𝗋𝗂𝗀𝗁𝗍 𝗂𝗌 𝗇𝗈𝗍, 𝗌𝗂𝗇𝖼𝖾 𝟣𝟤 𝗂𝗌 𝗍𝗈 𝗍𝗁𝖾 𝗅𝖾𝖿𝗍 𝗈𝖿 𝟪.

<table>
  <tr>
    <td>
      <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://github.com/AnastasKosstow/algorithms/blob/main/assets/trees/bst-dark.png">
        <img width="350" alt="heap" src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/trees/bst.png">
      </picture>
    </td>
    <td>
      <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://github.com/AnastasKosstow/algorithms/blob/main/assets/trees/nbst-dark.png">
        <img width="350" alt="heap" src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/trees/nbst.png">
      </picture>
    </td>
  </tr>
</table>





### Heap

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/heap.gif" width="270" alt="heap" />

> [!NOTE]
> 𝖠 𝗁𝖾𝖺𝗉 𝗂𝗌 𝖺 𝗌𝗉𝖾𝖼𝗂𝖺𝗅 𝖳𝗋𝖾𝖾-𝖻𝖺𝗌𝖾𝖽 𝖽𝖺𝗍𝖺 𝗌𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾 𝗂𝗇 𝗐𝗁𝗂𝖼𝗁 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗂𝗌 𝖺 𝖼𝗈𝗆𝗉𝗅𝖾𝗍𝖾 𝖡𝗂𝗇𝖺𝗋𝗒 𝖳𝗋𝖾𝖾 𝗂𝗇 𝗐𝗁𝗂𝖼𝗁 𝖾𝖺𝖼𝗁 𝗅𝖾𝗏𝖾𝗅 𝗁𝖺𝗌 𝖺𝗅𝗅 𝗈𝖿 𝗂𝗍𝗌 𝗇𝗈𝖽𝖾𝗌.
> <br>
> 𝖳𝗁𝖾 𝖾𝗑𝖼𝖾𝗉𝗍𝗂𝗈𝗇 𝗍𝗈 𝗍𝗁𝗂𝗌 𝗂𝗌 𝗍𝗁𝖾 𝖻𝗈𝗍𝗍𝗈𝗆 𝗅𝖾𝗏𝖾𝗅 𝗈𝖿 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾, 𝗐𝗁𝗂𝖼𝗁 𝗐𝖾 𝖿𝗂𝗅𝗅 𝗂𝗇 𝖿𝗋𝗈𝗆 𝗅𝖾𝖿𝗍 𝗍𝗈 𝗋𝗂𝗀𝗁𝗍.

#### Heap type
 - 𝖬𝗂𝗇 𝖧𝖾𝖺𝗉 - 𝗂𝖿 𝖾𝖺𝖼𝗁 𝗉𝖺𝗋𝖾𝗇𝗍 𝗇𝗈𝖽𝖾 𝗂𝗌 𝗅𝖾𝗌𝗌 𝗍𝗁𝖺𝗇 𝗈𝗋 𝖾𝗊𝗎𝖺𝗅 𝗍𝗈 𝗂𝗍𝗌 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 (𝗋𝗈𝗈𝗍 𝗂𝗌 𝖺𝗅𝗐𝖺𝗒𝗌 𝗍𝗁𝖾 𝗌𝗆𝖺𝗅𝗅𝖾𝗌𝗍 𝗂𝗍𝖾𝗆)
 - 𝖬𝖺𝗑 𝖧𝖾𝖺𝗉 - 𝗂𝖿 𝖾𝖺𝖼𝗁 𝗉𝖺𝗋𝖾𝗇𝗍 𝗇𝗈𝖽𝖾 𝗂𝗌 𝗀𝗋𝖾𝖺𝗍𝖾𝗋 𝗍𝗁𝖺𝗇 𝗈𝗋 𝖾𝗊𝗎𝖺𝗅 𝗍𝗈 𝗂𝗍𝗌 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 (𝗋𝗈𝗈𝗍 𝗂𝗌 𝖺𝗅𝗐𝖺𝗒𝗌 𝗍𝗁𝖾 𝗅𝖺𝗋𝗀𝖾𝗌𝗍 𝗂𝗍𝖾𝗆)

#### Representation
𝖣𝖾𝗌𝗉𝗂𝗍𝖾 𝖻𝖾𝗂𝗇𝗀 𝖼𝗈𝗇𝖼𝖾𝗉𝗍𝗎𝖺𝗅𝗅𝗒 𝖺 𝗍𝗋𝖾𝖾, 𝖺 𝗁𝖾𝖺𝗉 𝗂𝗌 𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖾𝖽 𝗎𝗌𝗂𝗇𝗀 𝖺𝗇 𝖺𝗋𝗋𝖺𝗒. 𝖳𝗁𝗂𝗌 𝗂𝗌 𝖾𝖿𝖿𝗂𝖼𝗂𝖾𝗇𝗍 𝗂𝗇 𝗍𝖾𝗋𝗆𝗌 𝗈𝖿 𝖻𝗈𝗍𝗁 𝗌𝗉𝖺𝖼𝖾 𝖺𝗇𝖽 𝗍𝗂𝗆𝖾.
<br>
𝖴𝗌𝗂𝗇𝗀 𝖺𝗇 𝖺𝗋𝗋𝖺𝗒 𝖾𝗅𝗂𝗆𝗂𝗇𝖺𝗍𝖾𝗌 𝗍𝗁𝖾 𝗇𝖾𝖾𝖽 𝖿𝗈𝗋 𝗉𝗈𝗂𝗇𝗍𝖾𝗋𝗌 𝗋𝖾𝗊𝗎𝗂𝗋𝖾𝖽 𝗂𝗇 𝖺 𝗍𝗋𝖾𝖾 𝗋𝖾𝗉𝗋𝖾𝗌𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇, 𝗍𝗁𝗎𝗌 𝗌𝖺𝗏𝗂𝗇𝗀 𝗌𝗉𝖺𝖼𝖾.
<br>

 - S𝗈 𝗁𝖾𝖺𝗉 𝗍𝗁𝖺𝗍 𝗅𝗈𝗈𝗄𝗌 𝗅𝗂𝗄𝖾 𝗍𝗁𝗂𝗌:

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/AnastasKosstow/algorithms/blob/main/assets/heap_tree_dark.png">
  <img width="500" alt="heap" src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/heap_tree_light.png">
</picture>

 - I𝗌 𝗋𝖾𝗉𝗋𝖾𝗌𝖾𝗇𝗍𝖾𝖽 𝗅𝗂𝗄𝖾 𝗍𝗁𝗂𝗌:

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/AnastasKosstow/algorithms/blob/main/assets/heap_arr_dark.png">
  <img width="500" alt="heap" src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/heap_arr_light.png">
</picture>

𝖧𝖾𝖺𝗉 𝗋𝗈𝗈𝗍 𝗂𝗌 *𝗂𝗇𝖽𝖾𝗑=𝟢* - 𝖿𝗂𝗋𝗌𝗍 𝗂𝗍𝖾𝗆 𝗈𝖿 𝗍𝗁𝖾 𝖺𝗋𝗋𝖺𝗒. 𝖣𝖾𝗉𝖾𝗇𝖽𝗂𝗇𝗀 𝗈𝗇 𝗍𝗁𝖾 𝗂𝗆𝗉𝗅𝖾𝗆𝖾𝗇𝗍𝖺𝗍𝗂𝗈𝗇 𝖬𝗂𝗇/𝖬𝖺𝗑, 𝗋𝗈𝗈𝗍 𝗂𝗌 𝖺𝗅𝗐𝖺𝗒𝗌 𝗍𝗁𝖾 𝗌𝗆𝖺𝗅𝗅𝖾𝗌𝗍/𝗅𝖺𝗋𝗀𝖾𝗌𝗍 𝗂𝗍𝖾𝗆 𝗈𝖿 𝗍𝗁𝖾 𝖺𝗋𝗋𝖺𝗒
<br>

𝖶𝗁𝖾𝗇 𝗎𝗌𝗂𝗇𝗀 𝖺𝗇 𝖺𝗋𝗋𝖺𝗒 𝗍𝗈 𝗋𝖾𝗉𝗋𝖾𝗌𝖾𝗇𝗍 𝖺 𝖻𝗂𝗇𝖺𝗋𝗒 𝗁𝖾𝖺𝗉, 𝗐𝖾 𝖺𝗋𝖾 𝖾𝗌𝗌𝖾𝗇𝗍𝗂𝖺𝗅𝗅𝗒 "𝖿𝗅𝖺𝗍𝗍𝖾𝗇𝗂𝗇𝗀" 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗌𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾 𝗂𝗇𝗍𝗈 𝖺 𝗅𝗂𝗌𝗍, 𝗐𝗁𝗂𝗅𝖾 𝗌𝗍𝗂𝗅𝗅 𝗆𝖺𝗂𝗇𝗍𝖺𝗂𝗇𝗂𝗇𝗀 𝗍𝗁𝖾 𝗉𝖺𝗋𝖾𝗇𝗍-𝖼𝗁𝗂𝗅𝖽 𝗋𝖾𝗅𝖺𝗍𝗂𝗈𝗇𝗌𝗁𝗂𝗉𝗌.
<br>

𝖳𝗁𝖾 𝖺𝗋𝗋𝖺𝗒 𝗂𝗌 𝗌𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾𝖽 𝗌𝗎𝖼𝗁 𝗍𝗁𝖺𝗍 𝖿𝗈𝗋 𝖺𝗇𝗒 𝗀𝗂𝗏𝖾𝗇 𝗂𝗇𝖽𝖾𝗑 *i* (𝗌𝗍𝖺𝗋𝗍𝗂𝗇𝗀 𝖺𝗍 𝟢), 𝗍𝗁𝖾 𝗅𝖾𝖿𝗍 𝖼𝗁𝗂𝗅𝖽 𝗂𝗌 𝖺𝗍 `𝟤*𝗂 + 𝟣` 𝖺𝗇𝖽 𝗍𝗁𝖾 𝗋𝗂𝗀𝗁𝗍 𝖼𝗁𝗂𝗅𝖽 𝗂𝗌 𝖺𝗍 `𝟤*𝗂 + 2`.
<br>
𝖢𝗈𝗇𝗏𝖾𝗋𝗌𝖾𝗅𝗒, 𝗍𝗁𝖾 𝗉𝖺𝗋𝖾𝗇𝗍 𝗈𝖿 𝖺𝗇𝗒 𝗇𝗈𝗇-𝗋𝗈𝗈𝗍 𝗇𝗈𝖽𝖾 𝖺𝗍 𝗂𝗇𝖽𝖾𝗑 *i* 𝗂𝗌 𝖺𝗍 `(𝗂-𝟣)/𝟤`.
<br>

> - 𝖠𝖽𝖽𝗂𝗇𝗀 𝖺𝗇 𝖨𝗍𝖾𝗆 (𝗉𝗎𝗌𝗁):
>   - 𝖶𝗁𝖾𝗇 𝗐𝖾 𝖺𝖽𝖽 𝖺 𝗇𝖾𝗐 𝗂𝗍𝖾𝗆 𝗍𝗈 𝖺 𝗁𝖾𝖺𝗉, 𝗐𝖾 𝖺𝗅𝗐𝖺𝗒𝗌 𝖺𝖽𝖽 𝗂𝗍 𝖺𝗍 𝗍𝗁𝖾 𝗏𝖾𝗋𝗒 𝖾𝗇𝖽 𝗈𝖿 𝗍𝗁𝖾 𝖺𝗋𝗋𝖺𝗒.
>     𝖳𝗁𝗂𝗌 𝖾𝗇𝗌𝗎𝗋𝖾𝗌 𝗍𝗁𝖺𝗍 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗋𝖾𝗆𝖺𝗂𝗇𝗌 𝖼𝗈𝗆𝗉𝗅𝖾𝗍𝖾 𝖺𝗇𝖽 𝖾𝗏𝖾𝗋𝗒 𝗅𝖾𝗏𝖾𝗅 𝗈𝖿 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗂𝗌 𝖿𝗂𝗅𝗅𝖾𝖽 𝖿𝗋𝗈𝗆 𝗅𝖾𝖿𝗍 𝗍𝗈 𝗋𝗂𝗀𝗁𝗍 𝗐𝗂𝗍𝗁𝗈𝗎𝗍 𝗀𝖺𝗉𝗌.
>   - 𝖠𝖿𝗍𝖾𝗋 𝖺𝖽𝖽𝗂𝗇𝗀 𝗍𝗁𝖾 𝗇𝖾𝗐 𝗂𝗍𝖾𝗆, 𝗐𝖾 "𝗁𝖾𝖺𝗉𝗂𝖿𝗒" 𝗂𝗍 𝖻𝗒 𝖼𝗈𝗆𝗉𝖺𝗋𝗂𝗇𝗀 𝗂𝗍 𝗐𝗂𝗍𝗁 𝗂𝗍𝗌 𝗉𝖺𝗋𝖾𝗇𝗍 𝗇𝗈𝖽𝖾 𝖺𝗇𝖽 𝗌𝗐𝖺𝗉𝗉𝗂𝗇𝗀 𝗂𝖿 𝗇𝖾𝖼𝖾𝗌𝗌𝖺𝗋𝗒 (𝗂𝗇 𝖺 𝗆𝖺𝗑-𝗁𝖾𝖺𝗉, 𝗂𝖿 𝗂𝗍'𝗌 𝗅𝖺𝗋𝗀𝖾𝗋 𝗍𝗁𝖺𝗇 𝗍𝗁𝖾 𝗉𝖺𝗋𝖾𝗇𝗍).
>     𝖳𝗁𝗂𝗌 𝗉𝗋𝗈𝖼𝖾𝗌𝗌 𝗂𝗌 𝗋𝖾𝗉𝖾𝖺𝗍𝖾𝖽 𝗎𝗇𝗍𝗂𝗅 𝗍𝗁𝖾 𝗇𝖾𝗐 𝗂𝗍𝖾𝗆 𝗂𝗌 𝗂𝗇 𝗍𝗁𝖾 𝖼𝗈𝗋𝗋𝖾𝖼𝗍 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇 𝗐𝗁𝖾𝗋𝖾 𝗂𝗍'𝗌 𝗇𝗈 𝗅𝗈𝗇𝗀𝖾𝗋 𝗀𝗋𝖾𝖺𝗍𝖾𝗋 𝗍𝗁𝖺𝗇 𝗂𝗍𝗌 𝗉𝖺𝗋𝖾𝗇𝗍.
> - 𝖱𝖾𝗆𝗈𝗏𝗂𝗇𝗀 𝖺𝗇 𝖨𝗍𝖾𝗆 (𝗉𝗈𝗉):
>   - 𝖶𝗁𝖾𝗇 𝗋𝖾𝗆𝗈𝗏𝖾𝗂𝗇𝗀 𝖺𝗇 𝗂𝗍𝖾𝗆, 𝗎𝗌𝗎𝖺𝗅𝗅𝗒 𝗐𝖾 𝗋𝖾𝗆𝗈𝗏𝖾 𝗍𝗁𝖾 𝗋𝗈𝗈𝗍, 𝖻𝖾𝖼𝖺𝗎𝗌𝖾 𝗍𝗁𝖺𝗍'𝗌 𝗍𝗁𝖾 𝗆𝖺𝗑𝗂𝗆𝗎𝗆 𝗂𝗍𝖾𝗆 𝗂𝗇 𝖺 𝗆𝖺𝗑-𝗁𝖾𝖺𝗉.
>   - 𝖳𝗈 𝗆𝖺𝗂𝗇𝗍𝖺𝗂𝗇 𝗍𝗁𝖾 𝖼𝗈𝗆𝗉𝗅𝖾𝗍𝖾 𝗍𝗋𝖾𝖾 𝗉𝗋𝗈𝗉𝖾𝗋𝗍𝗒, 𝗐𝖾 𝗍𝖺𝗄𝖾 𝗍𝗁𝖾 𝗅𝖺𝗌𝗍 𝗂𝗍𝖾𝗆 𝗂𝗇 𝗍𝗁𝖾 𝖺𝗋𝗋𝖺𝗒 𝖺𝗇𝖽 𝗆𝗈𝗏𝖾 𝗂𝗍 𝗍𝗈 𝗍𝗁𝖾 𝗋𝗈𝗈𝗍 𝗉𝗈𝗌𝗂𝗍𝗂𝗈𝗇.
>     𝖥𝗋𝗈𝗆 𝗍𝗁𝖾𝗋𝖾, 𝗐𝖾 "𝗁𝖾𝖺𝗉𝗂𝖿𝗒" 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝖻𝗒 𝗋𝖾𝗉𝖾𝖺𝗍𝖾𝖽𝗅𝗒 𝗌𝗐𝖺𝗉𝗉𝗂𝗇𝗀 𝗍𝗁𝗂𝗌 𝗇𝖾𝗐 𝗋𝗈𝗈𝗍 𝗐𝗂𝗍𝗁 𝗂𝗍𝗌 𝗅𝖺𝗋𝗀𝖾𝗌𝗍 𝖼𝗁𝗂𝗅𝖽 (𝗂𝖿 𝗂𝗍'𝗌 𝗌𝗆𝖺𝗅𝗅𝖾𝗋 𝗍𝗁𝖺𝗇 𝗂𝗍𝗌 𝖼𝗁𝗂𝗅𝖽𝗋𝖾𝗇) 𝗎𝗇𝗍𝗂𝗅 𝗍𝗁𝖾 𝗁𝖾𝖺𝗉 𝗉𝗋𝗈𝗉𝖾𝗋𝗍𝗒 𝗂𝗌 𝗋𝖾𝗌𝗍𝗈𝗋𝖾𝖽.

F𝗈r𝗆𝗎𝗅𝖺𝗌 𝖿𝗈𝗋 𝖿𝗂𝗇𝖽𝗂𝗇𝗀 parent and child nodes 𝖿𝗋𝗈𝗆 𝗁𝖾𝖺𝗉 𝗐𝗁𝗂𝖼𝗁 𝗂𝗌 𝗋𝖾𝗉𝗋𝖾𝗌𝖾𝗇𝗍𝖾𝖽 𝖺𝗌 𝖺𝗇 𝖺𝗋𝗋𝖺𝗒:
- 𝖥𝗂𝗇𝖽 𝗉𝖺𝗋𝖾𝗇𝗍 𝗇𝗈𝖽𝖾 - 𝗉𝖺𝗋𝖾𝗇𝗍(𝗂𝗇𝖽𝖾𝗑) = (𝗂𝗇𝖽𝖾𝗑 - 𝟣) / 𝟤
- 𝖥𝗂𝗇𝖽 𝗅𝖾𝖿𝗍 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 - 𝗅𝖾𝖿𝗍(𝗂𝗇𝖽𝖾𝗑) = 𝟤 * 𝗂𝗇𝖽𝖾𝗑 + 𝟣
- 𝖥𝗂𝗇𝖽 𝗋𝗂𝗀𝗁𝗍 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 - 𝗋𝗂𝗀𝗁𝗍(𝗂𝗇𝖽𝖾𝗑) = 𝟤 * 𝗂𝗇𝖽𝖾𝗑 + 𝟤

𝖥𝗈𝗋 𝖾𝗑𝖺𝗆𝗉𝗅𝖾:
- 𝖯𝖺𝗋𝖾𝗇𝗍 𝗇𝗈𝖽𝖾 𝗈𝖿 𝟩 𝗂𝗌 - 𝗉𝖺𝗋𝖾𝗇𝗍(𝟥) = (𝟥-𝟣) / 𝟤 = 𝟣 𝗌𝗈, 𝗉𝖺𝗋𝖾𝗇𝗍𝖭𝗈𝖽𝖾𝖨𝗇𝖽𝖾𝗑 = 𝟣 (𝟥 𝗂𝗌 𝗂𝗇𝖽𝖾𝗑 𝗈𝖿 𝗂𝗍𝖾𝗆 𝟩)
- 𝖫𝖾𝖿𝗍 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 𝗈𝖿 𝟪 𝗂𝗌, 𝗅𝖾𝖿𝗍(𝟣) = (𝟤 𝗑 𝟣) + 𝟣 = 𝟥 𝗌𝗈, 𝗅𝖾𝖿𝗍𝖢𝗁𝗂𝗅𝖽𝖭𝗈𝖽𝖾𝖨𝗇𝖽𝖾𝗑 = 𝟥 (𝗂𝗍𝖾𝗆: 𝟩)
- 𝖱𝗂𝗀𝗁𝗍 𝖼𝗁𝗂𝗅𝖽 𝗇𝗈𝖽𝖾 𝗈𝖿 𝟪 𝗂𝗌, 𝗋𝗂𝗀𝗁𝗍(𝟣) = (𝟤 𝗑 𝟣) + 𝟤 = 𝟦 𝗌𝗈, 𝗋𝗂𝗀𝗁𝗍𝖢𝗁𝗂𝗅𝖽𝖭𝗈𝖽𝖾𝖨𝗇𝖽𝖾𝗑 = 𝟦 (𝗂𝗍𝖾𝗆: 𝟣)

#### Operations
 - I𝗇𝗌𝖾𝗋𝗍: `Θ(𝗅𝗈𝗀 𝗇)`
 - D𝖾𝗅𝖾𝗍𝖾: `Θ(𝗅𝗈𝗀 𝗇)`
 - F𝗂𝗇𝖽 𝗆𝗂𝗇/𝗆𝖺𝗑: `𝖮(1)` (𝗍𝗁𝖾 𝗆𝖺𝗑𝗂𝗆𝗎𝗆/𝗆𝗂𝗇𝗂𝗆𝗎𝗆 𝗂𝗍𝖾𝗆 𝗂𝗌 𝖺𝗅𝗐𝖺𝗒𝗌 𝖺𝗍 𝗍𝗁𝖾 𝗋𝗈𝗈𝗍, 𝗆𝖺𝗄𝗂𝗇𝗀 𝗍𝗁𝗂𝗌 𝗈𝗉𝖾𝗋𝖺𝗍𝗂𝗈𝗇 𝗐𝗂𝗍𝗁 `O(1)` 𝗍𝗂𝗆𝖾 𝖼𝗈𝗆𝗉𝗅𝖾𝗑𝗂𝗍𝗒)

<b>𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</b> <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/heap.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BinaryHeap/Heap.cs">C#</a>

---

### Binary Search Tree

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/bst.gif" width="270" alt="bst" />

> [!NOTE]
> 𝖠 𝖡𝗂𝗇𝖺𝗋𝗒 𝖲𝖾𝖺𝗋𝖼𝗁 𝖳𝗋𝖾𝖾 𝗂𝗌 𝖺 𝗇𝗈𝖽𝖾-𝖻𝖺𝗌𝖾𝖽 𝖽𝖺𝗍𝖺 𝗌𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾 𝗐𝗁𝖾𝗋𝖾 𝖾𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝖼𝗈𝗇𝗍𝖺𝗂𝗇𝗌 𝖺 𝗄𝖾𝗒 𝖺𝗇𝖽 𝗍𝗐𝗈 𝗌𝗎𝖻𝗍𝗋𝖾𝖾𝗌, 𝗅𝖾𝖿𝗍 𝖺𝗇𝖽 𝗋𝗂𝗀𝗁𝗍.
> <br>
> 𝖳𝗁𝖾 𝗅𝖾𝖿𝗍 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝗈𝖿 𝖺 𝗇𝗈𝖽𝖾 𝖼𝗈𝗇𝗍𝖺𝗂𝗇𝗌 𝗈𝗇𝗅𝗒 𝗇𝗈𝖽𝖾𝗌 𝗐𝗂𝗍𝗁 𝗄𝖾𝗒𝗌 𝗅𝖾𝗌𝗌𝖾𝗋 𝗍𝗁𝖺𝗇 𝗍𝗁𝖾 𝗇𝗈𝖽𝖾’𝗌 𝗄𝖾𝗒. 𝖳𝗁𝖾 𝗋𝗂𝗀𝗁𝗍 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝗈𝖿 𝖺 𝗇𝗈𝖽𝖾 𝖼𝗈𝗇𝗍𝖺𝗂𝗇𝗌 𝗈𝗇𝗅𝗒 𝗇𝗈𝖽𝖾𝗌 𝗐𝗂𝗍𝗁 𝗄𝖾𝗒𝗌 𝗀𝗋𝖾𝖺𝗍𝖾𝗋 𝗍𝗁𝖺𝗇 𝗍𝗁𝖾 𝗇𝗈𝖽𝖾’𝗌 𝗄𝖾𝗒.

𝖫𝖾𝗍 *Ｔ* = 𝖻𝗂𝗇𝖺𝗋𝗒 𝗌𝖾𝖺𝗋𝖼𝗁 𝗍𝗋𝖾𝖾
<br>
𝖫𝖾𝗍 *Ｎ* = 𝗇𝗈𝖽𝖾
<br>
𝖥𝗈𝗋 𝖾𝗏𝖾𝗋𝗒 𝗇𝗈𝖽𝖾 *Ｎ* 𝗂𝗇 *Ｔ*, 𝗂𝖿 *Ｎ*.𝗅𝖾𝖿𝗍 𝖺𝗇𝖽 *Ｎ*.𝗋𝗂𝗀𝗁𝗍 𝖾𝗑𝗂𝗌𝗍, 𝗍𝗁𝖾𝗇:
- 𝖠𝗅𝗅 𝗏𝖺𝗅𝗎𝖾𝗌 𝗂𝗇 𝗍𝗁𝖾 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝗋𝗈𝗈𝗍𝖾𝖽 𝖺𝗍 *Ｎ*.𝗅𝖾𝖿𝗍 𝖺𝗋𝖾 𝗅𝖾𝗌𝗌 𝗍𝗁𝖺𝗇 *Ｎ*.𝗏𝖺𝗅𝗎𝖾.
- 𝖠𝗅𝗅 𝗏𝖺𝗅𝗎𝖾𝗌 𝗂𝗇 𝗍𝗁𝖾 𝗌𝗎𝖻𝗍𝗋𝖾𝖾 𝗋𝗈𝗈𝗍𝖾𝖽 𝖺𝗍 *Ｎ*.𝗋𝗂𝗀𝗁𝗍 𝖺𝗋𝖾 𝗀𝗋𝖾𝖺𝗍𝖾𝗋 𝗍𝗁𝖺𝗇 *Ｎ*.𝗏𝖺𝗅𝗎𝖾.

![binary search tree](https://quicklatex.com/cache3/ef/ql_e8c5e2d7ee26b7a124486e9202767eef_l3.png)

#### Operations
 - 𝖲𝖾𝖺𝗋𝖼𝗁: 𝖡𝖾𝖼𝖺𝗎𝗌𝖾 𝗈𝖿 𝗂𝗍𝗌 𝗈𝗋𝖽𝖾𝗋𝖾𝖽 𝗇𝖺𝗍𝗎𝗋𝖾, 𝗌𝖾𝖺𝗋𝖼𝗁𝗂𝗇𝗀 𝖿𝗈𝗋 𝖺𝗇 𝖾𝗅𝖾𝗆𝖾𝗇𝗍 𝗂𝗇 𝖺 𝖡𝖲𝖳 𝗂𝗌 𝗍𝗒𝗉𝗂𝖼𝖺𝗅𝗅𝗒 `Θ(𝗅𝗈𝗀 𝗇)` 𝗈𝗋 `𝖮(𝗇)` 𝗐𝗁𝖾𝗇 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗋𝖾𝗌𝖾𝗆𝖻𝗅𝖾𝗌 𝖺 𝗅𝗂𝗇𝗄𝖾𝖽 𝗅𝗂𝗌𝗍
 - 𝖨𝗇𝗌𝖾𝗋𝗍: 𝖳𝗒𝗉𝗂𝖼𝖺𝗅𝗅𝗒 `Θ(𝗅𝗈𝗀 𝗇)` 𝗐𝗁𝖾𝗇 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗋𝖾𝗆𝖺𝗂𝗇𝗌 𝗋𝖾𝖺𝗌𝗈𝗇𝖺𝖻𝗅𝗒 𝖻𝖺𝗅𝖺𝗇𝖼𝖾𝖽 𝗈𝗋 `𝖮(𝗇)` 𝗂𝖿 𝗍𝗁𝖾 𝗍𝗋𝖾𝖾 𝗂𝗌 𝗁𝗂𝗀𝗁𝗅𝗒 𝗎𝗇𝖻𝖺𝗅𝖺𝗇𝖼𝖾𝖽
 - 𝖣𝖾𝗅𝖾𝗍𝖾: 𝖲𝗂𝗆𝗂𝗅𝖺𝗋 𝗍𝗈 𝗌𝖾𝖺𝗋𝖼𝗁 𝖺𝗇𝖽 𝗂𝗇𝗌𝖾𝗋𝗍 - `Θ(𝗅𝗈𝗀 𝗇)` 𝗈𝗋 `𝖮(𝗇)` 𝖿𝗈𝗋 𝗁𝗂𝗀𝗁𝗅𝗒 𝗎𝗇𝖻𝖺𝗅𝖺𝗇𝖼𝖾𝖽 𝗍𝗋𝖾𝖾

<b>𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</b> <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/binary_search_tree.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BinarySearchTree/BinarySearchTree.cs">C#</a>

---

### LinkedList

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/list.gif" width="270" alt="list" />

> [!NOTE]
> 𝖫𝗂𝗇𝗄𝖾𝖽𝖫𝗂𝗌𝗍𝗌 𝖺𝗋𝖾 𝖺 𝗌𝖾𝗊𝗎𝖾𝗇𝖼𝖾 𝗈𝖿 𝗇𝗈𝖽𝖾𝗌, 𝖾𝖺𝖼𝗁 𝖼𝗈𝗇𝗍𝖺𝗂𝗇𝗂𝗇𝗀 𝖽𝖺𝗍𝖺 𝖺𝗇𝖽 𝖺 𝗋𝖾𝖿𝖾𝗋𝖾𝗇𝖼𝖾 (𝗅𝗂𝗇𝗄) 𝗍𝗈 𝗍𝗁𝖾 𝗇𝖾𝗑𝗍 𝗇𝗈𝖽𝖾.
> 𝖤𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝗍𝗒𝗉𝗂𝖼𝖺𝗅𝗅𝗒 𝖼𝗈𝗇𝗍𝖺𝗂𝗇𝗌 𝗍𝗐𝗈 𝗉𝖺𝗋𝗍𝗌:
> - *Data:* 𝖳𝗁𝖾 𝖺𝖼𝗍𝗎𝖺𝗅 𝗏𝖺𝗅𝗎𝖾 𝗈𝗋 𝗂𝗇𝖿𝗈𝗋𝗆𝖺𝗍𝗂𝗈𝗇 𝗍𝗁𝖺𝗍 𝗍𝗁𝖾 𝗇𝗈𝖽𝖾 𝗋𝖾𝗉𝗋𝖾𝗌𝖾𝗇𝗍𝗌.
> - *Pointer (or Link):* 𝖠 𝗋𝖾𝖿𝖾𝗋𝖾𝗇𝖼𝖾 𝗍𝗈 𝗍𝗁𝖾 𝗇𝖾𝗑𝗍 𝗇𝗈𝖽𝖾 𝗂𝗇 𝗍𝗁𝖾 𝗌𝖾𝗊𝗎𝖾𝗇𝖼𝖾. 𝖨𝗇 𝖺 𝗌𝗂𝗇𝗀𝗅𝗒 𝗅𝗂𝗇𝗄𝖾𝖽 𝗅𝗂𝗌𝗍, 𝖾𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝗉𝗈𝗂𝗇𝗍𝗌 𝗍𝗈 𝗂𝗍𝗌 𝗌𝗎𝖼𝖼𝖾𝗌𝗌𝗈𝗋, 𝗐𝗁𝗂𝗅𝖾 𝗂𝗇 𝖺 𝖽𝗈𝗎𝖻𝗅𝗒 𝗅𝗂𝗇𝗄𝖾𝖽 𝗅𝗂𝗌𝗍, 𝖾𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝗁𝖺𝗌 𝗉𝗈𝗂𝗇𝗍𝖾𝗋𝗌 𝗍𝗈 𝖻𝗈𝗍𝗁 𝗂𝗍𝗌 𝗉𝗋𝖾𝗏𝗂𝗈𝗎𝗌 𝖺𝗇𝖽 𝗇𝖾𝗑𝗍 𝗇𝗈𝖽𝖾.

#### Types of LinkedLists
 - *Singly Linked List*: 𝖤𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝗁𝖺𝗌 𝗈𝗇𝗅𝗒 𝗈𝗇𝖾 𝗉𝗈𝗂𝗇𝗍𝖾𝗋 𝗍𝗈 𝗍𝗁𝖾 𝗇𝖾𝗑𝗍 𝗇𝗈𝖽𝖾
 - *Doubly Linked List*: 𝖤𝖺𝖼𝗁 𝗇𝗈𝖽𝖾 𝗁𝖺𝗌 𝗍𝗐𝗈 𝗉𝗈𝗂𝗇𝗍𝖾𝗋𝗌, 𝗈𝗇𝖾 𝗍𝗈 𝗍𝗁𝖾 𝗇𝖾𝗑𝗍 𝗇𝗈𝖽𝖾 𝖺𝗇𝖽 𝗈𝗇𝖾 𝗍𝗈 𝗍𝗁𝖾 𝗉𝗋𝖾𝗏𝗂𝗈𝗎𝗌 𝗇𝗈𝖽𝖾
 - *Circular Linked List*: 𝖳𝗁𝖾 𝗅𝖺𝗌𝗍 𝗇𝗈𝖽𝖾 𝗉𝗈𝗂𝗇𝗍𝗌 𝖻𝖺𝖼𝗄 𝗍𝗈 𝗍𝗁𝖾 𝖿𝗂𝗋𝗌𝗍 𝗇𝗈𝖽𝖾, 𝖿𝗈𝗋𝗆𝗂𝗇𝗀 𝖺 𝖼𝗂𝗋𝖼𝗅𝖾 (𝗂𝗍 𝖼𝖺𝗇 𝖻𝖾 𝗌𝗂𝗇𝗀𝗅𝗒 𝗈𝗋 𝖽𝗈𝗎𝖻𝗅𝗒)

<b>𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</b> <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/linked_list.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/LinkedList/SinglyLinkedList.cs">C#</a>

---

### Disjoint-set

<img src="https://github.com/AnastasKosstow/algorithms/blob/main/assets/ufds.gif" width="270" alt="djs" />

> [!NOTE]
> 𝖣𝗂𝗌𝗃𝗈𝗂𝗇𝗍-𝗌𝖾𝗍 𝖣𝖺𝗍𝖺 𝖲𝗍𝗋𝗎𝖼𝗍𝗎𝗋𝖾 𝖺𝗅𝗌𝗈 𝗄𝗇𝗈𝗐𝗇 𝖺𝗌 𝖺 𝗎𝗇𝗂𝗈𝗇-𝖿𝗂𝗇𝖽, 𝗄𝖾𝖾𝗉𝗌 𝗍𝗋𝖺𝖼𝗄 𝗈𝖿 𝖺 𝗌𝖾𝗍 𝗈𝖿 𝖾𝗅𝖾𝗆𝖾𝗇𝗍𝗌 𝗉𝖺𝗋𝗍𝗂𝗍𝗂𝗈𝗇𝖾𝖽 𝗂𝗇𝗍𝗈 𝗌𝖾𝗏𝖾𝗋𝖺𝗅 𝗇𝗈𝗇-𝗈𝗏𝖾𝗋𝗅𝖺𝗉𝗉𝗂𝗇𝗀 𝗌𝗎𝖻𝗌𝖾𝗍𝗌. 

#### Characteristics
 - 𝖤𝖿𝖿𝗂𝖼𝗂𝖾𝗇𝗍 𝖿𝗈𝗋 𝗁𝖺𝗇𝖽𝗅𝗂𝗇𝗀 𝖾𝗊𝗎𝗂𝗏𝖺𝗅𝖾𝗇𝖼𝖾 𝗋𝖾𝗅𝖺𝗍𝗂𝗈𝗇𝗌 𝖺𝗇𝖽 𝖼𝗈𝗇𝗇𝖾𝖼𝗍𝖾𝖽 𝖼𝗈𝗆𝗉𝗈𝗇𝖾𝗇𝗍𝗌 𝗂𝗇 𝖺 𝗇𝖾𝗍𝗐𝗈𝗋𝗄.
 - 𝖢𝗈𝗆𝗆𝗈𝗇𝗅𝗒 𝗎𝗌𝖾𝖽 𝗂𝗇 𝖺𝗅𝗀𝗈𝗋𝗂𝗍𝗁𝗆𝗌 𝗍𝗁𝖺𝗍 𝗋𝖾𝗊𝗎𝗂𝗋𝖾 𝖿𝗋𝖾𝗊𝗎𝖾𝗇𝗍 𝖼𝗁𝖾𝖼𝗄𝗌 𝗈𝖿 𝗐𝗁𝖾𝗍𝗁𝖾𝗋 𝗍𝗐𝗈 𝖾𝗅𝖾𝗆𝖾𝗇𝗍𝗌 𝖺𝗋𝖾 𝗂𝗇 𝗍𝗁𝖾 𝗌𝖺𝗆𝖾 𝗌𝗎𝖻𝗌𝖾𝗍.

#### Operations
 - *Find*: 𝖣𝖾𝗍𝖾𝗋𝗆𝗂𝗇𝖾𝗌 𝗐𝗁𝗂𝖼𝗁 𝗌𝗎𝖻𝗌𝖾𝗍 𝖺 𝗉𝖺𝗋𝗍𝗂𝖼𝗎𝗅𝖺𝗋 𝖾𝗅𝖾𝗆𝖾𝗇𝗍 𝗂𝗌 𝗂𝗇. 𝖳𝗁𝗂𝗌 𝖼𝖺𝗇 𝖻𝖾 𝗎𝗌𝖾𝖽 𝖿𝗈𝗋 𝖽𝖾𝗍𝖾𝗋𝗆𝗂𝗇𝗂𝗇𝗀 𝗂𝖿 𝗍𝗐𝗈 𝖾𝗅𝖾𝗆𝖾𝗇𝗍𝗌 𝖺𝗋𝖾 𝗂𝗇 𝗍𝗁𝖾 𝗌𝖺𝗆𝖾 𝗌𝗎𝖻𝗌𝖾𝗍
 - *Union*: 𝖩𝗈𝗂𝗇𝗌 𝗍𝗐𝗈 𝗌𝗎𝖻𝗌𝖾𝗍𝗌 𝗂𝗇𝗍𝗈 𝖺 𝗌𝗂𝗇𝗀𝗅𝖾 𝗌𝗎𝖻𝗌𝖾𝗍

#### Efficiency
 - 𝖶𝗂𝗍𝗁 𝗈𝗉𝗍𝗂𝗆𝗂𝗓𝖺𝗍𝗂𝗈𝗇𝗌 𝗅𝗂𝗄𝖾 *𝗎𝗇𝗂𝗈𝗇 𝖻𝗒 𝗋𝖺𝗇𝗄* 𝖺𝗇𝖽 *𝗉𝖺𝗍𝗁 𝖼𝗈𝗆𝗉𝗋𝖾𝗌𝗌𝗂𝗈𝗇*, 𝗍𝗁𝖾 𝗍𝗂𝗆𝖾 𝖼𝗈𝗆𝗉𝗅𝖾𝗑𝗂𝗍𝗒 𝗈𝖿 𝖻𝗈𝗍𝗁 𝖥𝗂𝗇𝖽 𝖺𝗇𝖽 𝖴𝗇𝗂𝗈𝗇 𝗈𝗉𝖾𝗋𝖺𝗍𝗂𝗈𝗇𝗌 𝖼𝖺𝗇 𝖻𝖾 𝖻𝗋𝗈𝗎𝗀𝗁𝗍 𝖽𝗈𝗐𝗇 𝖼𝗅𝗈𝗌𝖾 𝗍𝗈 𝖼𝗈𝗇𝗌𝗍𝖺𝗇𝗍 𝗍𝗂𝗆𝖾, `O(α(n))`, where `α` 𝗂𝗌 𝗍𝗁𝖾 𝗂𝗇𝗏𝖾𝗋𝗌𝖾 𝖠𝖼𝗄𝖾𝗋𝗆𝖺𝗇𝗇 𝖿𝗎𝗇𝖼𝗍𝗂𝗈𝗇, 𝗐𝗁𝗂𝖼𝗁 𝗀𝗋𝗈𝗐𝗌 𝗏𝖾𝗋𝗒 𝗌𝗅𝗈𝗐𝗅𝗒 𝖺𝗇𝖽 𝗂𝗌 𝗉𝗋𝖺𝖼𝗍𝗂𝖼𝖺𝗅𝗅𝗒 𝖼𝗈𝗇𝗌𝗍𝖺𝗇𝗍 𝖿𝗈𝗋 𝖺𝗅𝗅 𝗋𝖾𝖺𝗌𝗈𝗇𝖺𝖻𝗅𝖾 𝗂𝗇𝗉𝗎𝗍 𝗌𝗂𝗓𝖾𝗌. 

<b>𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜:</b> <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/data_structures/disjoint_set.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/UnionFind/UnionFindSet.cs">C#</a>

---
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

---

### DFS (depth-first search)
> [!NOTE]
> Depth-First Search (DFS) is an algorithm used for traversing or searching tree or graph data structures. 
> It starts at a selected node (root) and explores as far as possible along each branch before backtracking.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Depth-first_search">Depth-first search</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dfs.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/DepthFirstSearch/GraphExtensions.cs">C#</a>

1. **Initialize:**
   - Start at the root node (or any node in a graph).
   - Create a `Stack` to keep track of the path.
   - Add the starting node to the `Stack` and mark it as visited.
2. **DFS Loop:**
   - While the `Stack` is not empty:
     - Pop a node from the `Stack`.
     - For each unvisited neighbor of this node:
       - Mark the neighbor as visited.
       - Add the neighbor to the `Stack`.
3. **Complete:**
   - Repeat until all reachable nodes are visited.
  
---

### BFS (breadth-first search)
> [!NOTE]
> Breadth-First Search (BFS) is an algorithm used for traversing or searching tree or graph data structures. 
> It starts at a selected node and explores all neighbor nodes at the present depth before moving on to nodes at the next depth level.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Breadth-first_search">Breadth-first search</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/bfs.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BreadthFirstSearch/GraphExtensions.cs">C#</a>

1. **Initialize:**
   - Start at the root node (or any node in a graph).
   - Create a `Queue` to keep track of the nodes to visit.
   - Add the starting node to the `Queue` and mark it as visited.
2. **BFS Loop:**
   - While the `Queue` is not empty:
     - Dequeue a node from the `Queue`.
     - For each unvisited neighbor of this node:
       - Mark the neighbor as visited.
       - Add the neighbor to the `Queue`.
3. **Complete:**
   - Repeat until all reachable nodes are visited.

---

### Dijkstra's algorithm
> [!NOTE]
> Dijkstra's Algorithm is a pathfinding algorithm that finds the shortest path from a starting node to all other nodes in a weighted graph.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm">Dijkstra's Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/dijkstra.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Dijkstra/GraphExtensions.cs">C#</a>

> 𝙲𝚘𝚖𝚙𝚞𝚝𝚎𝚛𝚙𝚑𝚒𝚕𝚎 - <a href="https://www.youtube.com/watch?v=GazC3A4OQTE">Dijkstra's Algorithm by Dr Michael Pound</a>

1. **Initialize:**
   - Set initial distances to all nodes as infinity, except the start node which should be zero.
   - Create a `priority queue` and add the start node with distance 0.
2. **Algorithm Loop:**
   - While the `priority queue` is not empty:
     - Remove the node with the smallest distance.
     - For each neighbor, calculate the new distance and update if it's smaller.
3. **Complete:**
   - All shortest paths from the start node are determined.

---

### Bellman-Ford algorithm
> [!NOTE]
> The Bellman-Ford algorithm is used for computing shortest paths in a weighted graph. Unlike Dijkstra's, it can handle graphs with negative weight edges.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm">Bellman-Ford Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/bellman_ford.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/BellmanFord/GraphExtensions.cs">C#</a>

1. **Initialize:**
   - Set the distance to the source as 0 and all other distances as infinite.
2. **Relaxation Loop:**
   - For each edge `(u, v)`, update the distance to v if a shorter path is found via u.
   - Repeat this for all edges `|V|-1` times (`|V|` is the number of vertices).
3. **Negative Cycle Check:**
   - Check for further distance improvements; if found, a negative cycle exists.

---

### Floyd Warshall algorithm
> [!NOTE]
> The Floyd-Warshall algorithm is used for finding shortest paths in a weighted graph with positive or negative edge weights (but with no negative cycles).

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm">Floyd Warshall Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/floyd_warshall.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Floyd%E2%80%93Warshall/GraphExtensions.cs">C#</a>

1. **Matrix Setup:**
   - Initialize a matrix with distances between all pairs of vertices. Set 0 for self-loops and infinity for no direct path.
2. **Dynamic Programming:**
   - Update the matrix to find the shortest distance between each pair using an intermediate vertex.
3. **Complete:**
   - The matrix now contains the shortest distances between all pairs of nodes.

---

### Kruskal algorithm
> [!NOTE]
> Kruskal's Algorithm is a minimum spanning tree algorithm that finds an edge subset of a connected, weighted graph
> that connects all the vertices together, without any cycles and with the minimum possible total edge weight.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Kruskal%27s_algorithm">Kruskal Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/kruskal.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Kruskal/GraphExtensions.cs">C#</a>

1. **Sort Edges:**
   - Sort all edges of the graph in non-decreasing order of their weight.
2. **Initialize Forest:**
   - Create a forest, initially with each vertex as an individual tree. (use Disjoint-set/Union-Find)
3. **Edge Selection:**
   - For each edge, check if it forms a cycle in the forest.
     - If not, add it to the forest.
4. **Complete:**
   - Continue until the forest has (V-1) edges (V is the number of vertices).

---

### Prim's algorithm
> [!NOTE]
> Prim's Algorithm is a minimum spanning tree algorithm used in a connected, weighted graph.
> It builds the spanning tree by adding the next cheapest vertex to the existing tree until all vertices are included.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Prim%27s_algorithm">Prim's Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/prim.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Prim/GraphExtensions.cs">C#</a>

1. **Initialize Priority Queue:**
   - Start from a root vertex and add all its edges to a `priority queue`.
2. **Select Cheapest Edge:**
   - Repeatedly choose the edge with the smallest weight that connects a vertex in the tree to a vertex outside.
3. **Check for Cycles:**
   - Ensure that adding the chosen edge doesn’t create a cycle. (use Disjoint-set/Union-Find)
4. **Expand the Tree:**
   - Add the selected edge and vertex to the `spanning tree`.
5. **Repeat:**
   - Continue the process until all vertices are included in the `spanning tree`.

---

### Kosaraju's algorithm
> [!NOTE]
> Kosaraju's algorithm is a depth-first search based method used to find strongly connected components in a directed graph.
> The algorithm involves two passes of depth-first search.
> The first pass is used to calculate finishing times of vertices, and the second pass identifies the strongly connected components based on these times.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm">Kosaraju's Algorithm</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/kosaraju.rs">Rust</a> - <a href="https://github.com/AnastasKosstow/algorithms/blob/main/csharp/Kosaraju/GraphExtensions.cs">C#</a>

1. **First DFS Pass:**
   - Perform a `depth-first search (DFS)` on the original graph.
   - Track the completion order of vertices and push them onto a `stack`.
2. **Second DFS Pass:**
   - Pop nodes from the `stack` in the order they were completed in the first `DFS`.
   - Perform `DFS` on the transposed graph starting from each popped node, if it hasn't been visited.
3. **Identify Strongly Connected Components:**
   - Each `DFS` call in the second pass identifies a strongly connected component.
   - Collect the nodes visited in each `DFS` call as a single SCC.
4. **Complete:**
   - The algorithm finishes when all vertices have been popped and processed in the second DFS pass. At this point, all SCCs in the graph have been identified.

---

### Tarjan's algorithm
> [!NOTE]
> Tarjan's algorithm is a depth-first search based algorithm used to find strongly connected components (SCCs) in a directed graph.
> An SCC is a component where every vertex is reachable from every other vertex in the same component.
> This algorithm is efficient and can find all SCCs in a graph in linear time.

- 𝙲𝚘𝚗𝚌𝚎𝚙𝚝: Understand the <a href="https://emre.me/algorithms/tarjans-algorithm/#articulation-point">Tarjan's SCCs Algorithm (emre.me)</a>
- 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: <a href="https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/tarjan.rs">Rust</a> - <a href="">C#</a>

1. **Initialize:**
   - Assign a unique integer index to each node, initially undefined.
   - Define a low-link value for each node, initially set to its index.
2. **Graph Traversal**
   - Increment discovery time for each visited node.
   - Store discovery time and initial low-link value for each node.
3. **DFS Loop:**
   - For each node, count its children and track its parent.
   - Apply DFS recursively to unvisited successors.
   - Update the node's low-link value based on children's low-link values.
4. **Handling Back Edges**
   - Update the low-link value of the current node based on the discovery time of previously visited nodes that are not the parent.
5. **Repeat:**
   - Repeat this process for all nodes in the graph.
6. **Complete:**
   - The algorithm terminates when all nodes have been processed.

<br>

<table>
  <thead>
    <tr align="left">
      <th><details>
<summary>

#### `Open section` -> *Tarjan's* algorithm use cases

</summary>

  ### Articulation Points
  To find articulation points using Tarjan's algorithm, an additional step of identifying vertices that, if removed, increase the number of connected components is needed. 
  
  - 𝙴𝚡𝚝𝚎𝚗𝚍𝚎𝚍 𝚂𝚝𝚎𝚙: After completing the DFS loop, check each node. If it's a root node with two or more children, it's an articulation point. For other nodes, if the low-link value of a child is greater than       or equal to the index of the node, then this node is an articulation point.
  - 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: [Rust](https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/tarjan_articulation_points.rs) | C#
  
  ### Subgraph Components
  To find subgraph components using Tarjan's algorithm, it's essential to focus on grouping nodes into their respective SCCs.
  
  - 𝙴𝚡𝚝𝚎𝚗𝚍𝚎𝚍 𝚂𝚝𝚎𝚙: Upon finishing the DFS for a node, if the node's low-link value equals its index, it indicates the start of a new SCC. Collect all nodes explored since then into a separate SCC group.
  - 𝙸𝚖𝚙𝚕𝚎𝚖𝚎𝚗𝚝𝚊𝚝𝚒𝚘𝚗𝚜: [Rust](https://github.com/AnastasKosstow/algorithms/blob/main/rust/src/graphs/tarjan_subgraph_components.rs) | C#

</details></th>
    </tr>
  </thead>
</table>

