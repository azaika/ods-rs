mod sl_list;
mod se_list;

pub use sl_list::*;
pub use se_list::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
