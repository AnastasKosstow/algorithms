using AVLTree;

var avl = new AVLTree<int>();
avl.Insert(10);
avl.Insert(5);
Console.WriteLine(avl.Root.Value);
avl.Insert(1);
Console.WriteLine(avl.Root.Value);
