pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet { 
            parent: (0..size).collect(),
            rank: vec![0; size]
        }
    }

    pub fn find(&mut self, index: usize) -> usize {
        if self.parent[index] != index {
            self.parent[index] = self.find(self.parent[index]);
        }
        self.parent[index]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        }
        else if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        }
        else {
            self.parent[y_root] = x_root;
            self.rank[x_root] += 1;
        }
    }
}
