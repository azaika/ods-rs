mod backing_array;
mod array_stack;
mod array_queue;
mod array_deque;

pub use backing_array::*;
pub use array_stack::*;
pub use array_queue::*;
pub use array_deque::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
