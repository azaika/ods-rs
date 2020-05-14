use crate::backing_array::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStack<T : Default + Clone> {
    arr : BackingArray<T>,
    n : usize
}

impl<T: Default + Clone> ArrayStack<T> {
    pub fn new() -> Self {
        Self { arr : BackingArray::new(), n : 0 }
    }

    pub fn with_size(n : usize) -> Self {
        Self { arr : BackingArray::with_size(n), n : n }
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    fn check_idx(&self, i : usize) -> bool {
        i < self.size()
    }

    pub fn get(&self, i : usize) -> Option<&T> {
        if self.check_idx(i) { Some(&self.arr[i]) } else { None }
    }

    pub fn set(&mut self, i : usize, x : T) -> Option<T> {
        if !self.check_idx(i) {
            return None;
        }

        let old = self.arr[i].clone();
        self.arr[i] = x;

        Some(old)
    }
}