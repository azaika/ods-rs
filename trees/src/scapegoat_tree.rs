
#[derive(Debug)]
struct Node<K, T> {
    key : K,
    value : T,
    size : usize,
    left : Option<Box<Self>>,
    right : Option<Box<Self>>,
    parent : *mut Self
}

#[derive(Debug)]
pub struct ScapegoatTree<K : Ord, V> {
    root : Option<Box<Node<K, V>>>,
    n : usize,
    q : usize
}

impl<K : Ord, V> ScapegoatTree<K, V> {
    pub fn new() -> Self {
        Self { root : None, n : 0, q : 0 }
    }

    pub fn from_vec(src : Vec<(K, V)>) -> Self {
        let mut tree = Self::new();

        for (key, value) in src {
            tree.insert(key, value);
        }

        tree
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn locate(&self, key : &K) -> (Option<&Box<Node<K, V>>>, usize) {
        if self.is_empty() {
            return (None, 0);
        }

        let mut node = self.root.as_ref().unwrap();
        let mut depth = 1;

        loop {
            let next;
            if *key < node.key {
                next = &node.left;
            }
            else if *key > node.key {
                next = &node.right;
            }
            else {
                break;
            }

            if next.is_none() {
                return (Some(node), depth);
            }
            
            node = next.as_ref().unwrap();
            depth += 1;
        }

        (Some(node), depth)
    }
    fn locate_mut(&mut self, key : &K) -> (Option<&mut Box<Node<K, V>>>, usize) {
        if self.is_empty() {
            return (None, 0)
        }

        let mut node = self.root.as_mut().unwrap();
        let mut depth = 1;

        while *key != node.key {
            if *key < node.key {
                if let Some(ref mut next) = node.left {
                    node = next;
                    depth += 1;
                    continue;
                }
            }
            else {
                if let Some(ref mut next) = node.right {
                    node = next;
                    depth += 1;
                    continue;
                }
            }

            return (Some(node), depth);
        }

        (Some(node), depth)
    }

    pub fn get(&self, key : &K) -> Option<&V> {
        let (node, _) = self.locate(key);
        if node.is_none() {
            return None;
        }

        let node = node.unwrap();

        if node.key == *key {
            Some(&node.value)
        }
        else {
            None
        }
    }

    pub fn get_mut(&mut self, key : &K) -> Option<&mut V> {
        let (node, _) = self.locate_mut(key);
        if node.is_none() {
            return None;
        }

        let node = node.unwrap();

        if node.key == *key {
            Some(&mut node.value)
        }
        else {
            None
        }
    }

    unsafe fn recalc_size(node : *mut Node<K, V>) {
        if node != std::ptr::null_mut() {
            (*node).size = (*node).left.as_ref().map(|a| a.size).unwrap_or(0) + (*node).right.as_ref().map(|a| a.size).unwrap_or(0);
            Self::recalc_size((*node).parent);
        }
    }

    pub fn insert(&mut self, key : K, value : V) -> bool {
        if self.is_empty() {
            self.root = Some(Box::new(Node{ key : key, value : value, size : 1, left : None, right : None, parent : std::ptr::null_mut() }));
            self.n += 1;

            return true;
        }

        let q = self.q;

        let (node, depth) = self.locate_mut(&key);
        let node = node.unwrap();

        if node.key == key {
            return false;
        }

        let node_ptr : *mut Node<K, V> = node.as_mut();
        let next;
        if key < node.key {
            next = &mut node.left;
        }
        else {
            next = &mut node.right;
        }

        *next = Some(Box::new(Node{ key : key, value : value, size : 1, left : None, right : None, parent : node_ptr }));

        unsafe {
            Self::recalc_size(node_ptr);
        }

        if depth > ((q + 1) as f64).log(1.5) as usize + 1 {
            unsafe {
                let mut w = node_ptr;
                let mut a = (*w).size;
                let mut b = if (*w).parent != std::ptr::null_mut() { (*(*w).parent).size } else { 0 };

                while 3*a <= 2*b {
                    if (*w).parent == std::ptr::null_mut() { panic!() }

                    w = (*w).parent;

                    if (*w).parent == std::ptr::null_mut() { panic!() }

                    a = (*w).size;
                    b = (*(*w).parent).size;
                }

                w = (*w).parent;

                if w == std::ptr::null_mut() { panic!() }

                if (*w).parent != std::ptr::null_mut() && (*(*w).parent).left.as_ref().map(|a| a.key == node.key).unwrap_or(false) {
                    let a = self.take_box(&mut *w);
                    self.rebuild(a, true);
                }
                else {
                    let a = self.take_box(&mut *w);
                    self.rebuild(a, false);
                }
            }
        }

        self.n += 1;
        self.q += 1;

        true
    }

    fn splice(node : &mut Node<K, V>) -> Option<&mut Node<K, V>> {
        let mut child = if node.left.is_some() { node.left.take() } else { node.right.take() };

        if let Some(child) = child.as_mut() {
            child.parent = node.parent;
        }

        if node.parent != std::ptr::null_mut() {
            let parent;
            unsafe {
                parent = &mut *node.parent;
            }
            if parent.left.as_ref().map(|a| a.key == node.key).unwrap_or(false) {
                parent.left = child;
            }
            else {
                parent.right = child;
            }

            Some(parent)
        }
        else {
            None
        }
    }

    pub fn remove(&mut self, key : &K) -> bool {
        let (node, _) = self.locate_mut(&key);
        if node.is_none() || node.as_ref().unwrap().key != *key {
            return false;
        }

        let node = node.unwrap();
        if node.left.is_none() || node.right.is_none() {
            let node = Self::splice(node);
            if let Some(node) = node {
                unsafe {
                    Self::recalc_size(node);
                }
            }
        }
        else {
            let mut s = node.right.as_mut().unwrap();
            while let Some(ref mut next) = s.left {
                s = next;
            }

            std::mem::swap(&mut node.key, &mut s.key);
            std::mem::swap(&mut node.value, &mut s.value);

            let s = Self::splice(s);
            if let Some(s) = s {
                unsafe {
                    Self::recalc_size(s);
                }
            }
        }

        self.n -= 1;

        if self.q > 2*self.n {
            if self.is_empty() {
                self.q = self.n;
            }
            else {
                let r = self.root.take();
                self.rebuild(r, false);
            }
        }

        true
    }

    fn node_size(node : &Option<Box<Node<K, V>>>) -> usize {
        if let Some(ref node) = node {
            1 + Self::node_size(&node.left) + Self::node_size(&node.right)
        }
        else {
            0
        }
    }

    fn take_box(&mut self, node : &mut Node<K, V>) -> Option<Box<Node<K, V>>> {
        if node.parent == std::ptr::null_mut() {
            self.root.take()
        }
        else {
            let parent;
            unsafe {
                parent = &mut *node.parent;
            }
            if parent.left.as_ref().map(|a| a.key == node.key).unwrap_or(false) {
                parent.left.take()
            }
            else {
                parent.right.take()
            }
        }
    }

    fn into_vec_impl(dst : &mut Vec<Option<Box<Node<K, V>>>>, node : Option<Box<Node<K, V>>>, mut i : usize) -> usize {
        if let Some(mut node) = node {
            let l = node.left.take();
            let r = node.right.take();

            i = Self::into_vec_impl(dst, l, i);
            dst[i] = Some(node);
            return Self::into_vec_impl(dst, r, i + 1);
        }
        else {
            i
        }
    }

    fn into_vec(node : Option<Box<Node<K, V>>>) -> Vec<Option<Box<Node<K, V>>>> {
        let mut dst = Vec::new();
        dst.resize_with(Self::node_size(&node), Default::default);

        Self::into_vec_impl(&mut dst, node, 0);

        dst
    }
    
    fn rebuild_balanced(arr : &mut [Option<Box<Node<K, V>>>]) -> Option<Box<Node<K, V>>> {
        let len = arr.len();
        let i = len / 2;
        let mut node = arr[i].take();

        let mut left = if i != 0 { Self::rebuild_balanced(&mut arr[0..i]) } else { None };
        let mut right = if i + 1 < len { Self::rebuild_balanced(&mut arr[(i+1)..len]) } else { None };

        if let Some(ref mut l) = left {
            l.parent = node.as_mut().unwrap().as_mut();
        }
        if let Some(ref mut r) = right {
            r.parent = node.as_mut().unwrap().as_mut();
        }

        node.as_mut().unwrap().size = left.as_ref().map(|a| a.size).unwrap_or(0) + right.as_ref().map(|a| a.size).unwrap_or(0);
        node.as_mut().unwrap().left = left;
        node.as_mut().unwrap().right = right;
        
        node
    }

    fn rebuild(&mut self, node : Option<Box<Node<K, V>>>, is_left : bool) {
        if node.is_none() {
            return;
        }
        
        let parent;
        if node.as_ref().unwrap().parent == std::ptr::null_mut() {
            parent = None;
        }
        else {
            unsafe { parent = Some(&mut (*node.as_ref().unwrap().parent)); }
        }
        
        let mut balanced = Self::rebuild_balanced(&mut Self::into_vec(node));
        
        if let Some(par) = parent {
            balanced.as_mut().unwrap().parent = par;

            if is_left {
                par.left = balanced;
            }
            else {
                par.right = balanced;
            }
        }
        else {
            self.root = balanced;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scapegoat_tree_works() {
        let mut bst : ScapegoatTree<i32, i32> = ScapegoatTree::new();
        
        bst.insert(0, 0);
        bst.insert(5, 5);
        bst.insert(-2, -2);

        assert_eq!(bst.get(&0), Some(&0));
        assert_eq!(bst.get(&5), Some(&5));
        assert_eq!(bst.get(&-2), Some(&-2));

        assert_eq!(bst.get(&3), None);

        bst.remove(&0);

        assert_eq!(bst.get(&0), None);
        assert_eq!(bst.get(&5), Some(&5));
        assert_eq!(bst.get(&-2), Some(&-2));

        bst = ScapegoatTree::from_vec(vec![(1, 1), (10, 10), (8, 8), (9, 9), (7, 7), (0, 0)]);

        bst.remove(&5);
        bst.remove(&-2);

        assert_eq!(bst.get(&7), Some(&7));
        assert_eq!(bst.get(&0), Some(&0));

        for i in 0..64 {
            bst.insert(i, i);
        }

        for i in 0..64 {
            assert_eq!(bst.get(&i), Some(&i));
        }
    }
}