use std::ptr;

#[derive(Debug)]
struct Node<T : Default> {
    value : T,
    next : Option<Box<Self>>
}

#[derive(Debug)]
pub struct SLList<T : Default> {
    head : Option<Box<Node<T>>>,
    tail : *mut Box<Node<T>>,
    n : usize
}

impl<T : Default> SLList<T> {
    pub fn new() -> Self {
        Self{ head : None, tail : ptr::null_mut(), n : 0 }
    }

    pub fn push_front(&mut self, x : T) {
        let mut node = Box::new(Node {
            value : x,
            next : self.head.take()
        });
        
        if self.n == 0 {
            self.tail = node.next.as_mut().unwrap();
        }
        
        self.n += 1;
        self.head = Some(node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let Node{ value, next } = *self.head.take().unwrap();
        
        self.head = next;
        self.n -= 1;

        if self.n == 0 {
            self.tail = ptr::null_mut();
        }
        
        Some(value)
    }

    pub fn push_back(&mut self, x : T) {
        let mut node = Box::new(Node {
            value : x,
            next : self.head.take()
        });

        let node_ptr : *mut _ = &mut node;

        if self.n == 0 {
            self.head = Some(node);
        }
        else {
            unsafe {
                (*self.tail).next = Some(node);
            }
        }

        self.tail = node_ptr;
        self.n += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}