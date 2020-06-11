
#[derive(Debug)]
struct Node<K, T> {
    key : K,
    value : T,
    left : Option<Box<Self>>,
    right : Option<Box<Self>>,
    parent : *mut Self
}

#[derive(Debug)]
pub struct BSTree<K : Ord, V> {
    root : Option<Box<Node<K, V>>>,
    n : usize
}

impl<K : Ord, V> BSTree<K, V> {
    pub fn new() -> Self {
        BSTree { root : None, n : 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn locate(&self, key : &K) -> Option<&Box<Node<K, V>>> {
        if self.is_empty() {
            return None
        }

        let mut node = self.root.as_ref().unwrap();

        loop {
            let next;
            if *key > node.key {
                next = &node.left;
            }
            else if *key < node.key {
                next = &node.right;
            }
            else {
                break;
            }

            if next.is_none() {
                return Some(node);
            }
            
            node = next.as_ref().unwrap();
        }

        Some(node)
    }
    fn locate_mut(&mut self, key : &K) -> Option<&mut Box<Node<K, V>>> {
        if self.is_empty() {
            return None
        }

        let mut node = self.root.as_mut().unwrap();

        while *key != node.key {
            if *key < node.key {
                if let Some(ref mut next) = node.left {
                    node = next;
                    continue;
                }
            }
            else {
                if let Some(ref mut next) = node.right {
                    node = next;
                    continue;
                }
            }

            return Some(node);
        }

        Some(node)
    }

    pub fn get(&self, key : &K) -> Option<&V> {
        let node = self.locate(key);
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
        let node = self.locate_mut(key);
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

    pub fn insert(&mut self, key : K, value : V) -> bool {
        if self.is_empty() {
            self.root = Some(Box::new(Node{ key : key, value : value, left : None, right : None, parent : std::ptr::null_mut() }));
            return true;
        }

        let node = self.locate_mut(&key).unwrap();
        if node.key == key {
            return false;
        }

        let node_ptr : *mut Node<K, V> = node.as_mut();
        let next;
        if node.key < key {
            next = &mut node.left;
        }
        else {
            next = &mut node.right;
        }

        *next = Some(Box::new(Node{ key : key, value : value, left : None, right : None, parent : node_ptr }));
        
        true
    }

    fn splice(node : &mut Node<K, V>) {
        let mut child = if node.left.is_some() { node.left.take() } else { node.right.take() };

        if let Some(child) = child.as_mut() {
            child.parent = node.parent;
        }

        if node.parent != std::ptr::null_mut() {
            let parent;
            unsafe {
                parent = &mut *node.parent;
            }
            if node.key < parent.key {
                parent.left = child;
            }
            else {
                parent.right = child;
            }
        }

        return;
    }

    pub fn remove(&mut self, key : &K) -> bool {
        let node = self.locate_mut(&key);
        if node.is_none() || node.as_ref().unwrap().key != *key {
            return false;
        }

        let node = node.unwrap();
        if node.left.is_none() || node.right.is_none() {
            Self::splice(node);
        }
        else {
            let mut s = node.right.as_mut().unwrap();
            while let Some(ref mut next) = s.left {
                s = next;
            }

            std::mem::swap(&mut node.key, &mut s.key);
            std::mem::swap(&mut node.value, &mut s.value);

            Self::splice(s);
        }

        self.n -= 1;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}