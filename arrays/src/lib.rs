mod backing_array;
mod array_stack;
mod array_queue;

pub use backing_array::*;
pub use array_stack::*;
pub use array_queue::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
