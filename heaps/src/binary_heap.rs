
#[derive(Debug)]
pub struct BinaryHeap<T : Ord> {
    src : Vec<T>,
    n : usize
}

impl<T : Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        Self{ src : vec![], n : 0 }
    }

    pub fn from_vec(v : Vec<T>) -> Self {
        let len = v.len();
        let mut heap = Self{ src : v, n : len };

        for i in (0..(len/2)).rev() {
            heap.trickle_down(i);
        }

        return heap;
    }

    pub fn into_vec(self) -> Vec<T> {
        self.src
    }

    fn left(idx : usize) -> usize {
        (idx << 1) + 1
    }
    fn right(idx : usize) -> usize {
        (idx << 1) + 2
    }
    fn parent(idx : usize) -> usize {
        (idx - 1) >> 1
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn bubble_up(&mut self) {
        if self.n == 0 {
            unreachable!();
        }
        if self.n == 1 {
            return;
        }

        let mut i = self.n - 1;
        let mut p = Self::parent(i);

        while i > 0 && self.src[i] < self.src[p] {
            self.src.swap(i, p);
            i = p;

            if p != 0 {
                p = Self::parent(p);
            }
        }
    }

    pub fn insert(&mut self, x : T) {
        self.src.push(x);
        self.n += 1;

        self.bubble_up();
    }

    fn trickle_down(&mut self, i : usize) {
        let mut i = i;
        while Self::left(i) < self.n {
            let l = Self::left(i);
            let r = Self::right(i);

            if r >= self.n {
                if self.src[i] > self.src[l] {
                    self.src.swap(i, l);
                }

                break;
            }

            if self.src[l] < self.src[r] {
                if self.src[i] > self.src[l] {
                    self.src.swap(i, l);
                    i = l;
                }
                else {
                    break;
                }
            }
            else {
                if self.src[i] > self.src[r] {
                    self.src.swap(i, r);
                    i = r;
                }
                else {
                    break;
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.src.swap(self.n - 1, 0);

        let x = self.src.pop();

        self.n -= 1;

        self.trickle_down(0);

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_heap_works() {
        let mut heap = BinaryHeap::new();

        for i in vec![5, 3, 4, 9, -1, 0] {
            heap.insert(i);
        }

        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), None);

        heap = BinaryHeap::from_vec(vec![5, 3, 4, 9, -1, 0]);

        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), None);
    }
}