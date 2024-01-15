pub struct DisjointSet {
    parent: Vec<usize>
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet { parent: (0..size).collect() }
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
        if x_root != y_root {
            self.parent[x_root] = y_root;
        }
    }
}
