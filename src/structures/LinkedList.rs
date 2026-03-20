use crate::structures::Node::Node;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
   pub fn add(&mut self, value: T) -> bool {
    if self.is_empty() {
        self.head = Some(Box::new(Node::new(value)));
    } else {
        let mut current = self.head.as_deref_mut().unwrap();
        loop {
            if current.get_next().is_none() {
                current.set_next(Node::new(value));
                break;
            }
            current = current.get_next_mut().unwrap();
        }
    }
    true
}
    pub fn size(&self) -> usize {
    let mut count = 0;
    let mut current = self.head.as_deref();
    while let Some(node) = current {
        count += 1;
        current = node.get_next().map(|n| n.as_ref());
    }
    count
}
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
