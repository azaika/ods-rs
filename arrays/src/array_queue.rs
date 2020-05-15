use crate::backing_array::*;
use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayQueue<T : Default + Clone> {
    arr : BackingArray<T>,
    n : usize,
    head : usize
}

impl<T: Default + Clone> ArrayQueue<T> {
    pub fn new() -> Self {
        Self { arr : BackingArray::new(), n : 0, head : 0 }
    }

    pub fn with_size(n : usize) -> Self {
        Self { arr : BackingArray::with_size(n), n : n, head : 0 }
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    fn resize(&mut self) {
        let mut new_arr = BackingArray::with_size(cmp::max(1, 2 * self.n));

        let len = self.arr.len();
        for i in 0..self.n {
            new_arr[i] = self.arr[(self.head + i) % len].clone();
        }

        self.arr = new_arr;
        self.head = 0;
    }

    pub fn add(&mut self, x : T) {
        if self.n == self.arr.len() {
            self.resize();
        }

        let len = self.arr.len();
        self.arr[(self.head + self.n) % len] = x;

        self.n += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let x = self.arr[self.head].clone();

        self.head = (self.head + 1) % self.arr.len();
        self.n -= 1;
        
        if self.arr.len() >= 3 * self.n {
            self.resize()
        }
        
        Some(x)
    }
}
