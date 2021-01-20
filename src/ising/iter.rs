/// A weird fact about Rust standard library is there seems to be nothing that implements `Iterator` and iterates
/// over numbers instead of their references. The result is that it is impossible to get an arbitrary `Iterator` 
/// filled with indexes of sites to be flipped, or, in other words, we cannot have a generalized version of `Range`
/// that contains arbitrary numbers instead of a sequence of successive numbers. 

pub struct IndexRange {
    indexes: Box<Vec<usize>>
}

impl IndexRange {
    pub fn new<'a>(indexes: impl Iterator<Item = &'a usize>) -> Self {
        Self {
            indexes: Box::new(indexes.map(|&x| {x}).collect())
        }
    }
}

impl Iterator for IndexRange {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.indexes.pop()
    }
}
