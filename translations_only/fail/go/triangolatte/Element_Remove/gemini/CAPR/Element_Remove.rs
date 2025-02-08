
pub struct Element {
    pub prev: Option<Box<Element>>,
    pub next: Option<Box<Element>>,
}

impl Element {
    pub fn remove(&mut self) {
        if let Some(next) = self.next.as_mut() {
            next.prev = self.prev.take();
        }
        if let Some(prev) = self.prev.as_mut() {
            prev.next = self.next.take();
        }
    }
}
