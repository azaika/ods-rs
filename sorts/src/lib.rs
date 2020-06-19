use heaps::BinaryHeap;

pub fn heap_sort<T : Ord>(arr : Vec<T>) -> Vec<T> {
    let mut sorted = Vec::with_capacity(arr.len());
    let mut heap = BinaryHeap::from_vec(arr);
    while !heap.is_empty() {
        sorted.push(heap.pop().unwrap());
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_works() {
        assert_eq!(heap_sort(vec![5, 3, 4, 9, -1, 0]), vec![-1, 0, 3, 4, 5, 9]);
    }
}
