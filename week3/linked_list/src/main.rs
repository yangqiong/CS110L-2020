use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    // Make LinkedList generic
    let mut list: LinkedList<String> = LinkedList::new();
    list.push_front("a".to_string());
    list.push_front("b".to_string());
    list.push_front("c".to_string());
    list.push_front("d".to_string());
    println!("{}", list);

    let mut list2 = list.clone();
    list2.push_front("e".to_string());
    println!("list: {}", list);
    println!("list2: {}", list2);

    println!("list is equal to list2: {}", list == list2);
    list2.pop_front();
    println!("list is equal to list2: {}", list == list2);

    for val in &list {
        print!("{}", val);
    }
    println!("");
    println!("original list : {}", list);

    let mut v2 = list.into_iter();
    println!("{:?}", v2.next());
}
