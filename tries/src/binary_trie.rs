use std::ptr;
use num::traits::*;

#[derive(Debug)]
struct LeafNode<T> {
    value : T,
    prev : *mut Self,
    next : *mut Self,
    parent : *mut InnerNode<T>
}

#[derive(Debug)]
struct InnerNode<T> {
    child : [Option<Box<Node<T>>>; 2],
    parent : *mut Self,
    jump : *mut LeafNode<T>
}

#[derive(Debug)]
enum Node<T> {
    Inner(InnerNode<T>),
    Leaf(LeafNode<T>)
}

impl<T> Node<T> {
    fn as_inner(&mut self) -> &mut InnerNode<T> {
        if let Node::Inner(inner) = self { inner } else { panic!(); }
    }
    fn as_leaf(&mut self) -> &mut LeafNode<T> {
        if let Node::Leaf(leaf) = self { leaf } else { panic!(); }
    }
}

#[derive(Debug)]
pub struct BinaryTrie<T : Unsigned + PrimInt> {
    root : Node<T>,
    dummy : LeafNode<T>,
    n : usize
}

impl<T : Unsigned + PrimInt + std::fmt::Display> BinaryTrie<T> {
    const BITWISE: usize = std::mem::size_of::<T>() * 8;

    pub fn new() -> Self {
        let mut ret = Self {
            root : Node::Inner(InnerNode {
                child : [None, None],
                parent : ptr::null_mut(),
                jump : ptr::null_mut()
            }),
            dummy : LeafNode {
                value : T::zero(),
                prev : ptr::null_mut(),
                next : ptr::null_mut(),
                parent : ptr::null_mut()
            },
            n : 0
        };

        let ptr : *mut LeafNode<T> = &mut ret.dummy;
        ret.dummy.prev = ptr;
        ret.dummy.next = ptr;

        ret
    }

    // returns (node_ptr, depth, is_right)
    fn find_node(&self, x : T) -> (&Node<T>, usize, bool) {
        let mut c : bool = false;

        let mut u = &self.root;

        for i in 0..Self::BITWISE {
            c = ((x >> (Self::BITWISE - i - 1)) & T::one()).is_zero();

            if let Node::Inner(inner) = u {
                if let Some(ref ch) = inner.child[if c {1} else {0}] {
                    u = ch.as_ref();
                }
                else {
                    return (u, i, c);
                }
            }
        }

        (u, Self::BITWISE, c)
    }

    // returns (node_ptr, depth, is_right)
    fn find_node_mut(&mut self, x : T) -> (&mut Node<T>, usize, bool) {
        let mut c : bool = false;

        let mut u = &mut self.root;
        let mut ptr : *mut Node<T> = u;

        for i in 0..Self::BITWISE {
            c = ((x >> (Self::BITWISE - i - 1)) & T::one()).is_zero();

            ptr = u;

            if let Node::Inner(inner) = u {
                if let Some(ref mut ch) = inner.child[if c {1} else {0}] {
                    u = ch.as_mut();
                }
                else {
                    return unsafe {(&mut *ptr, i, c)};
                }
            }
        }

        unsafe {(&mut *ptr, Self::BITWISE, c)}
    }

    pub fn has(&self, x : T) -> bool {
        let (node, _, _) = self.find_node(x);

        if let Node::Leaf(_) = node { true } else { false }
    }

    pub fn lower_bound(&self, x : T) -> Option<T> {
        let (node, _, is_right) = self.find_node(x);
        
        match node {
            Node::Inner(inner) => unsafe {
                let u : *const LeafNode<T> = if is_right { (*inner.jump).next } else { inner.jump };
                
                if u == &self.dummy { None } else { Some((*u).value) }
            },
            Node::Leaf(_) => Some(x)
        }
    }

    pub fn insert(&mut self, x : T) -> bool {
        let (init_node, depth, is_right) = self.find_node_mut(x);

        match init_node {
            Node::Leaf(_) => false,
            Node::Inner(inner) => {
                let pred : *mut LeafNode<T> = unsafe {
                    if is_right { inner.jump } else { (*inner.jump).prev }
                };

                inner.jump = ptr::null_mut();
                
                let mut par_ptr : *mut InnerNode<T> = inner;
                let mut node = &mut inner.child[if is_right {1} else {0}];
                for i in depth..(Self::BITWISE - 1) {
                    *node = Some(Box::new(Node::Inner(InnerNode{
                        child : [None, None],
                        parent : par_ptr,
                        jump : ptr::null_mut()
                    })));

                    let new_par = node.as_mut().unwrap().as_mut().as_inner();
                    
                    let c = ((x >> (Self::BITWISE - i - 1)) & T::one()).is_zero();
                    unsafe {
                        node = &mut (*par_ptr).child[if c {1} else {0}];
                    }

                    par_ptr = new_par;
                }

                *node = Some(Box::new(Node::Leaf(LeafNode{
                    value : x,
                    prev : pred,
                    next : unsafe { (*pred).next },
                    parent : par_ptr
                })));

                let leaf = node.as_mut().unwrap().as_mut().as_leaf();

                unsafe {
                    (*pred).next = leaf;
                    (*(*pred).next).prev = leaf;
                }

                while par_ptr != ptr::null_mut() {
                    let par : &mut InnerNode<T>;
                    let l;
                    let r;
                    unsafe {
                        par = &mut *par_ptr;
                    
                        l = par.child[0].is_none() && (par.jump == ptr::null_mut() || (*par.jump).value > x);
                        r = par.child[1].is_none() && (par.jump == ptr::null_mut() || (*par.jump).value < x);
                    }
                            
                    par.jump = if l || r { leaf } else { ptr::null_mut() };

                    par_ptr = par.parent;
                }

                self.n += 1;
                true
            }
        }
    }

    pub fn remove(&mut self, x : T) -> bool {
        let (node, mut depth, mut is_right) = self.find_node_mut(x);

        let mut par_ptr;
        let next_leaf;
        let prev_leaf;
        if let Node::Leaf(leaf) = node {
            depth -= 1;
            par_ptr = leaf.parent;
            next_leaf = leaf.next;
            prev_leaf = leaf.prev;

            unsafe {
                (*leaf.prev).next = next_leaf;
                (*leaf.next).prev = prev_leaf;

                while (*par_ptr).jump != ptr::null_mut() && (*(*par_ptr).jump).value == x {
                    if (*par_ptr).parent == ptr::null_mut() {
                        break;
                    }

                    depth -= 1;
                    par_ptr = (*par_ptr).parent;
                    is_right = (*(*par_ptr).child[0].as_mut().unwrap().as_mut().as_inner().jump).value == x;
                }
            }
        }
        else {
            return false;
        }

        let del = par_ptr;

        loop {
            let node = unsafe { &mut *par_ptr };

            if node.jump != ptr::null_mut() && unsafe {&mut *node.jump}.value == x {
                let c = ((x >> (Self::BITWISE - depth - 1)) & T::one()).is_zero();
                node.jump = if c { prev_leaf } else { next_leaf };
            }

            par_ptr = node.parent;

            if depth == 0 {
                break;
            }
            depth -= 1;
        }

        unsafe {&mut *del}.child[if is_right {1} else {0}] = None;

        self.n -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_trie_works() {
        let mut trie : BinaryTrie<u8> = BinaryTrie::new();

        trie.insert(5);
        trie.insert(128);
        trie.insert(<u8 as Bounded>::max_value());
        trie.insert(72);

        println!("1");

        assert_eq!(trie.has(8), false);
        assert_eq!(trie.has(72), true);

        println!("2");

        assert_eq!(trie.lower_bound(5), Some(5));
        assert_eq!(trie.lower_bound(6), Some(128));
        assert_eq!(trie.lower_bound(0), Some(5));
        assert_eq!(trie.lower_bound(<u8 as Bounded>::max_value()), Some(<u8 as Bounded>::max_value()));

        println!("3");

        assert_eq!(trie.remove(42), false);

        assert_eq!(trie.remove(5), true);
        assert_eq!(trie.remove(<u8 as Bounded>::max_value()), true);

        println!("4");
        
        assert_eq!(trie.lower_bound(0), Some(6));
        assert_eq!(trie.lower_bound(128), Some(128));
        assert_eq!(trie.lower_bound(129), None);
    }
}
