use std::ptr;
use std::mem;
use arrays::ArrayDeque;

type BoundedDeque<T> = ArrayDeque<T>;

struct Node<T : Default + Clone> {
    block : BoundedDeque<T>,
    next : Option<Box<Self>>,
    prev : *mut Self
}

impl<T : Default + Clone> Node<T> {
    pub fn new() -> Self {
        Self { block : BoundedDeque::new(), prev : ptr::null_mut(), next : None }
    }
    pub fn with_capacity(capacity : usize) -> Self {
        Self { block : BoundedDeque::with_capacity(capacity), prev : ptr::null_mut(), next : None }
    }
}

pub struct SEList<T : Default + Clone> {
    dummy : Box<Node<T>>, // dummy = (head, None, tail)
    n : usize,
    block_size : usize
}

impl<T : Default + Clone> SEList<T> {
    pub fn new() -> Self {
        Self { dummy : Box::new(Node::new()), n : 0, block_size : 4 }
    }
    
    fn head(&self) -> & Node<T> {
        if self.dummy.as_ref().prev == ptr::null_mut() {
            panic!("SEList::head_mut() was called for empty list.");
        }

        self.dummy.as_ref().next.as_ref().unwrap().as_ref()
    }
    fn head_mut(&mut self) -> &mut Node<T> {
        if self.dummy.as_ref().prev == ptr::null_mut() {
            panic!("SEList::head_mut() was called for empty list.");
        }

        self.dummy.as_mut().next.as_mut().unwrap().as_mut()
    }
    fn last_mut(&mut self) -> &mut Node<T> {
        if self.dummy.as_ref().prev == ptr::null_mut() {
            panic!("SEList::last_mut() was called for empty list.");
        }

        unsafe {
            &mut *self.dummy.as_mut().prev
        }
    }

    // idx must satisfy idx < n
    fn get_location(&self, idx : usize) -> (*const Node<T>, usize) {
        if idx < self.n/2 {
            let mut rem = idx;
            let mut ptr : *const _ = self.head();

            unsafe {
                while rem >= (*ptr).block.size() {
                    rem -= (*ptr).block.size();
                    ptr = (*ptr).next.as_ref().unwrap().as_ref();
                }
            }
            
            (ptr, rem)
        }
        else {
            let mut ptr : *const _ = self.dummy.as_ref();
            let mut cur = self.n;

            unsafe {
                while cur > idx {
                    ptr = (*ptr).prev;
                    cur -= (*ptr).block.size();
                }
            }

            (ptr, idx - cur)
        }
    }
    fn get_location_mut(&mut self, idx : usize) -> (*mut Node<T>, usize) {
        if idx < self.n/2 {
            let mut rem = idx;
            let mut ptr : *mut _ = self.head_mut();

            unsafe {
                while rem >= (*ptr).block.size() {
                    rem -= (*ptr).block.size();
                    ptr = (*ptr).next.as_mut().unwrap().as_mut();
                }
            }

            (ptr, rem)
        }
        else {
            let mut ptr : *mut _ = self.dummy.as_mut();
            let mut cur = self.n;

            unsafe {
                while cur > idx {
                    ptr = (*ptr).prev;
                    cur -= (*ptr).block.size();
                }
            }

            (ptr, idx - cur)
        }
    }

    pub fn get(&self, idx : usize) -> Option<&T> {
        if idx >= self.n {
            return None;
        }

        let (node_ptr, idx) = self.get_location(idx);

        assert_ne!(node_ptr, ptr::null());

        unsafe {
            (*node_ptr).block.get(idx)
        }
    }
    pub fn get_mut(&mut self, idx : usize) -> Option<&mut T> {
        if idx >= self.n {
            return None;
        }

        let (node_ptr, idx) = self.get_location_mut(idx);

        assert_ne!(node_ptr, ptr::null_mut());

        unsafe {
            (*node_ptr).block.get_mut(idx)
        }
    }

    pub fn set(&mut self, idx : usize, x : T) -> Option<T>{
        if idx >= self.n {
            return None;
        }

        let mut x = x;

        let (node_ptr, idx) = self.get_location_mut(idx);
        assert_ne!(node_ptr, ptr::null_mut());
        unsafe {
            mem::swap(&mut x, (*node_ptr).block.get_mut(idx).unwrap());
        }

        Some(x)
    }

    fn insert_node(pos : &mut Box<Node<T>>, block_size : usize) {
        let mut new_node = Node::with_capacity(block_size);
        new_node.prev = pos.as_mut();
        new_node.next = pos.as_mut().next.take();

        pos.as_mut().next = Some(Box::new(new_node));

        let new_node : *mut _ = pos.as_mut().next.as_mut().unwrap();
        unsafe {
            if let Some(next) = (*new_node).as_mut().next.as_mut() {
                next.as_mut().prev = (*new_node).as_mut();
            }
        }
    }
    fn push_back_node(&mut self) {
        unsafe {
            Self::insert_node((*self.last_mut().prev).next.as_mut().unwrap(), self.block_size);
        }
        self.dummy.prev = self.last_mut().next.as_mut().unwrap().as_mut();
    }

    pub fn push_back(&mut self, x : T) {
        if self.n == 0 {
            Self::insert_node(&mut self.dummy, self.block_size);
            self.dummy.prev = self.dummy.next.as_mut().unwrap().as_mut();
        }
        else if self.last_mut().block.size() == self.block_size {
            self.push_back_node();
        }

        self.last_mut().block.push_back(x);
        self.n += 1;
    }

    unsafe fn spread(&mut self, node : *mut Node<T>) {
        let mut cur = node;
        for _ in 0..(self.block_size-1) {
            cur = (*cur).next.as_mut().unwrap().as_mut();
        }

        Self::insert_node(&mut (*(*cur).prev).next.as_mut().unwrap(), self.block_size);
        if self.dummy.as_ref().prev == cur {
            self.dummy.as_mut().prev = (*cur).next.as_mut().unwrap().as_mut();
        }

        cur = (*cur).next.as_mut().unwrap().as_mut();

        while cur != node {
            let prev = (*cur).prev;

            while (*cur).block.size() <= self.block_size - 1 {
                (*cur).block.push_front((*prev).block.pop_back().unwrap());
            }

            cur = prev;
        }
    }

    pub fn add(&mut self, idx : usize, x : T) {
        if idx >= self.n {
            if idx == self.n {
                self.push_back(x);
            }
            return;
        }

        let (origin_node, idx) = self.get_location_mut(idx);

        let mut node_ptr = origin_node;
        let mut r = 0;
        unsafe {
            while r < self.block_size && (*node_ptr).block.size() == self.block_size {
                r += 1;
                
                if node_ptr != self.dummy.as_mut().prev {
                    node_ptr = (*node_ptr).next.as_mut().unwrap().as_mut();
                }
                else {
                    break;
                }
            }
        }

        if r == self.block_size { // next b blocks are full
            unsafe {
                self.spread(origin_node);
            }
        }
        else {
            if node_ptr == self.dummy.as_mut().prev && self.last_mut().block.size() == self.block_size { // come to last and last is full
                self.push_back_node();
                node_ptr = self.dummy.prev;
            }

            while node_ptr != origin_node {
                unsafe {
                    let cur = &mut *node_ptr;
                    cur.block.push_front((*cur.prev).block.pop_back().unwrap());
    
                    node_ptr = cur.prev;
                }
            }
        }

        unsafe {
            (*origin_node).block.add(idx, x);
        }
        self.n += 1;
    }

    fn remove_node(node : *mut Node<T>) {
        unsafe {
            let next = (*node).next.take();

            let prev_node = (*node).prev;
            (*prev_node).next = next;
            // node has dead
            if let Some(next) = (*prev_node).next.as_mut() {
                next.as_mut().prev = prev_node;
            }
        }
    }

    unsafe fn gather(&mut self, node : *mut Node<T>) {
        let mut cur = node;
        
        for _ in 0..(self.block_size - 1) {
            let next = (*cur).next.as_mut().unwrap().as_mut();

            while (*cur).block.size() <= self.block_size - 1 {
                (*cur).block.push_back(next.block.pop_front().unwrap());
            }

            cur = next;
        }
        
        if cur == self.dummy.as_mut().prev {
            self.dummy.as_mut().prev = (*cur).prev;
        }
        Self::remove_node(cur)
    }

    pub fn remove(&mut self, idx : usize) -> Option<T> {
        if idx >= self.n {
            return None;
        }

        let (origin_node, idx) = self.get_location_mut(idx);

        let old : Option<T>;

        let mut dist_node = origin_node;
        let mut r = 0;
        unsafe {
            while r < self.block_size && (*dist_node).block.size() < self.block_size {
                r += 1;
                
                if dist_node != self.dummy.as_mut().prev {
                    dist_node = (*dist_node).next.as_mut().unwrap().as_mut();
                }
                else {
                    break;
                }
            }
        }

        if r == self.block_size {
            unsafe {
                self.gather(origin_node);
                old = (*origin_node).block.remove(idx);
            }
        }
        else {
            unsafe {
                old = (*origin_node).block.remove(idx);
            }

            let mut node_ptr = origin_node;
            unsafe {
                while node_ptr != dist_node {
                    let cur = &mut *node_ptr;
                    cur.block.push_back(cur.next.as_mut().unwrap().block.pop_front().unwrap());
    
                    node_ptr = cur.next.as_mut().unwrap().as_mut();
                }
                if (*dist_node).block.size() == 0 {
                    if dist_node == self.dummy.as_mut().prev {
                        self.dummy.as_mut().prev = (*dist_node).prev;
                    }
                    Self::remove_node(dist_node);
                }
            }
        }

        self.n -= 1;

        if self.n == 0 {
            self.dummy.prev = ptr::null_mut();
        }

        old
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn se_list_works() {
        let mut list = SEList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), Some(&4));

        list.add(0, -1);
        list.add(0, -2);
        assert_eq!(list.get(0), Some(&-2));
        assert_eq!(list.get(1), Some(&-1));
        assert_eq!(list.get(2), Some(&1));

        list.add(2, 0);
        assert_eq!(list.get(1), Some(&-1));
        assert_eq!(list.get(2), Some(&0));
        assert_eq!(list.get(3), Some(&1));

        assert_eq!(list.remove(2), Some(0));
        assert_eq!(list.get(0), Some(&-2));
        assert_eq!(list.get(1), Some(&-1));
        assert_eq!(list.get(2), Some(&1));

        assert_eq!(list.remove(0), Some(-2));
        assert_eq!(list.remove(0), Some(-1));
        assert_eq!(list.remove(0), Some(1));
        assert_eq!(list.remove(0), Some(2));
        assert_eq!(list.remove(0), Some(3));
        assert_eq!(list.remove(0), Some(4));
        assert_eq!(list.remove(0), None);

        for i in 0i32..36 {
            list.add(0, 35 - i);
        }
        for i in 0i32..36 {
            assert_eq!(list.get(i as usize), Some(&i));
        }
        for i in 0..36 {
            assert_eq!(list.remove(0), Some(i));
        }
    }
}