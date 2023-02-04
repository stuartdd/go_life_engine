mod lists;
mod lists_tests;
use crate::lists::lists::*;
fn main() {
    let llist = LinkedLists::new();

    println!("{}", llist);
}
