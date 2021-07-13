struct UnionFindSet {
    n: usize,
    innerSet: Vec<i32>,
}

impl unionFindSet {
    pub fn new(n: usize) -> UnionFindSet {
        UnionFindSet {
            n,
            innerSet: (0..n).collect(),
        }
    }

    pub  find(&self, x : i32) ->bool {

        if x !=  self.innerSet[x] {
            self.innerSet[x] = Self::find(self.innerSet[x]);
        }
        return self.innerSet[x]
    }
}

fn main() {
    println!("Hello, world!");
}
