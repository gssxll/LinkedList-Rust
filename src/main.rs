mod structures;
use crate::structures::LinkedList::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.add(1);   
    list.add(2);
    list.add(3);
    println!("Size of the list: {}", list.size());
    println!("Is the list empty? {}", list.is_empty());
}