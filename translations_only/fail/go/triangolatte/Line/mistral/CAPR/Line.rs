

use std::boxed::Box;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn append(head: &mut Option<Box<Node>>, value: i32) {
    match head {
        Some(ref mut node) => {
            let mut current = node;
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node {
                value,
                next: None,
            }));
        }
        None => {
            *head = Some(Box::new(Node {
                value,
                next: None,
            }));
        }
    }
}

fn main() {}

