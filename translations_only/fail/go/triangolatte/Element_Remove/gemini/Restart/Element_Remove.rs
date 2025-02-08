
pub struct Element {
    pub prev: Option<Box<Element>>,
    pub next: Option<Box<Element>>,
}

impl Element {
    pub fn remove(&mut self) {
        self.next.as_mut().map(|next| next.prev = self.prev.take());
        self.prev.as_mut().map(|prev| prev.next = self.next.take());
    }
}
