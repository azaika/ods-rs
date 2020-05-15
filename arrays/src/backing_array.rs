use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackingArray<T : Default + Clone> {
    arr : Vec<T>
}

impl<T : Default + Clone> BackingArray<T> {
    pub fn new() -> Self {
        Self{ arr : Vec::<T>::new() }
    }
    
    pub fn with_size(n : usize) -> Self {
        Self{ arr : vec![Default::default(); n]  }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }
}

impl<T : Default + Clone> Index<usize> for BackingArray<T> {
    type Output = T;

    fn index(&self, idx : usize) -> &Self::Output {
        &self.arr[idx]
    }
}
impl<T : Default + Clone> IndexMut<usize> for BackingArray<T> {
    fn index_mut(&mut self, idx : usize) -> &mut Self::Output {
        &mut self.arr[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a1 = BackingArray::<i32>::new();

        assert_eq!(a1.len(), 0);

        let mut a2 = BackingArray::<i32>::with_size(5);

        assert_eq!(a2.len(), 5);

        a2[0] = 1;
        a2[1] = 2;
        a2[2] = 3;
        a2[3] = 2;
        a2[4] = 1;

        assert_eq!(a2[0], a2[4]);
        assert_eq!(a2[1], a2[3]);
    }
}
