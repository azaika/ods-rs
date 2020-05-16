use crate::backing_array::*;
use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayDeque<T : Default + Clone> {
    arr : BackingArray<T>,
    n : usize,
    head : usize
}

impl<T: Default + Clone> ArrayDeque<T> {
    pub fn new() -> Self {
        Self { arr : BackingArray::new(), n : 0, head : 0 }
    }

    pub fn with_size(n : usize) -> Self {
        Self { arr : BackingArray::with_size(n), n : n, head : 0 }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    fn check_idx(&self, i : usize) -> bool {
        i < self.size()
    }

    pub fn get(&self, idx : usize) -> Option<&T> {
        let len = self.arr.len();
        if self.check_idx(idx) { Some(&self.arr[(self.head + idx) % len]) } else { None }
    }

    pub fn get_mut(&mut self, idx : usize) -> Option<&mut T> {
        let len = self.arr.len();
        if self.check_idx(idx) { Some(&mut self.arr[(self.head + idx) % len]) } else { None }
    }

    pub fn set(&mut self, idx : usize, x : T) -> Option<T> {
        if !self.check_idx(idx) {
            return None;
        }

        let idx = (self.head + idx) % self.arr.len();
        let old = self.arr[idx].clone();
        self.arr[idx] = x;

        Some(old)
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

    pub fn add(&mut self, idx : usize, x : T) {
        if idx > self.size() {
            return;
        }

        if self.n == self.arr.len() {
            self.resize();
        }

        let len = self.arr.len();
        if idx < self.n/2 {
            self.head = if self.head == 0 {self.arr.len() - 1 } else { self.head - 1 };
            
            if idx != 0 {
                for i in 0..(idx-1) {
                    self.arr[(self.head + i) % len] = self.arr[(self.head + i + 1) % len].clone();
                }
            }
        }
        else {
            for i in (idx..self.n).rev() {
                self.arr[(self.head + i + 1) % len] = self.arr[(self.head + i) % len].clone();
            }
        }

        self.arr[(self.head + idx) % len] = x;

        self.n += 1;
    }

    pub fn remove(&mut self, idx : usize) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let len = self.arr.len();
        let x = self.arr[(self.head + idx) % len].clone();

        if idx < self.n/2 {
            if idx != 0 {
                for i in (0..(idx-1)).rev() {
                    self.arr[(self.head + i + 1) % len] = self.arr[(self.head + i) % len].clone();
                }
            }

            self.head = (self.head + 1) % len;
        }
        else {
            for i in idx..(self.n-1) {
                self.arr[(self.head + i) % len] = self.arr[(self.head + i + 1) % len].clone();
            }
        }

        self.n -= 1;
        
        if self.arr.len() >= 3 * self.n {
            self.resize()
        }
        
        Some(x)
    }
}
