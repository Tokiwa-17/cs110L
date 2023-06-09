use linked_list::LinkedList;
pub mod linked_list;
use linked_list::ComputeNorm;

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    list.push_front("hello".to_string());
    list.push_front("world".to_string());
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    println!("clone trait...");
    let mut list2 = LinkedList::new();
    list2.push_front("hello".to_string());
    list2.push_front("world".to_string());
    let mut list3 = list2.clone();
    println!("{}", list3);

    println!("PartialEq trait...");
    println!("list: {}", list);
    println!("list2: {}", list2);
    println!("list3: {}", list3);
    println!("list == list2: {}", list == list2);
    println!("list2 == list3: {}", list3 == list2);

    println!("ComputeNorm trait...");
    let mut list4 = LinkedList::new();
    list4.push_front(1.0);
    list4.push_front(2.0);
    list4.push_front(3.0);
    println!("list4 = {}", list4);
    println!("computeNorm: {}", list4.compute_norm());
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
