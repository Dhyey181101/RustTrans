
use std::boxed::Box;

pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
        }
    }
}
