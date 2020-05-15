use crate::backing_array::*;
use std::cmp;

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
        self.n
    }

    fn check_idx(&self, i : usize) -> bool {
        i < self.size()
    }

    pub fn get(&self, i : usize) -> Option<&T> {
        if self.check_idx(i) { Some(&self.arr[i]) } else { None }
    }
    
    pub fn get_mut(&mut self, i : usize) -> Option<&mut T> {
        if self.check_idx(i) { Some(&mut self.arr[i]) } else { None }
    }

    pub fn set(&mut self, idx : usize, x : T) -> Option<T> {
        if !self.check_idx(idx) {
            return None;
        }

        let old = self.arr[idx].clone();
        self.arr[idx] = x;

        Some(old)
    }

    fn resize(&mut self) {
        let mut new_arr = BackingArray::with_size(cmp::max(1, 2 * self.n));

        for i in 0..self.n {
            new_arr[i] = self.arr[i].clone();
        }

        self.arr = new_arr;
    }

    pub fn add(&mut self, idx : usize, x : T) {
        if !self.check_idx(idx) {
            return;
        }

        if self.n == self.arr.len() {
            self.resize();
        }

        for i in (idx..self.n).rev() {
            self.arr[i + 1] = self.arr[i].clone();
        }

        self.arr[idx] = x;
        self.n += 1;
    }

    pub fn remove(&mut self, idx : usize) -> Option<T> {
        if !self.check_idx(idx) {
            return None;
        }

        let x = self.arr[idx].clone();
        for i in idx..(self.n-1) {
            self.arr[i + 1] = self.arr[i].clone();
        }

        self.n -= 1;
        if self.arr.len() >= 3 * self.n {
            self.resize()
        }
        
        Some(x)
    }
}
