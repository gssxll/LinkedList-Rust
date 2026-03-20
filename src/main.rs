mod structures;
use crate::structures::LinkedList::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.add(1);   
    list.add(2);
    list.add(3);
    println!("Size of the list: {}", list.size());
    println!("Is the list empty? {}", list.is_empty());

    println!("List:");
    for value in list.iterator() {
        println!("{}", value);
    }

    println!("Contains 2? {}", list.contains(&2));
    println!("Contains 100? {}", list.contains(&100));

    println!("Remove 2: {}", list.remove(&2) );
    

    println!("List after remove:");
    for value in list.iterator() {
        println!("{}", value);
    }

    println!("List size after remove: {}", list.size());
}