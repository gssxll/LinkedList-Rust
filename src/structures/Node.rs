
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
    pub fn set_next(&mut self, node: Node<T>) {
        self.next = Some(Box::new(node));
    }
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn get_next(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
    pub fn get_next_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.next.as_mut()
    }
    pub fn take_next(&mut self) -> Option<Box<Node<T>>> {
    self.next.take()
    }
    pub fn next_mut(&mut self) -> &mut Option<Box<Node<T>>> {
    &mut self.next
    }
}