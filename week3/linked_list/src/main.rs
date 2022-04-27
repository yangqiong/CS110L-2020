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
    assert_eq!(list.get_size(), 4);
}
