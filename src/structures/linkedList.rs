pub struct LinkedList<T> {
    head:Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    pub fn add(&mut self, value: T) -> bool {
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(value)));
        }else{
            let mut current = self.head.as_mut().unwrap();
            while let Some(ref mut next) = current.next {
                current = next;
            }
            current.set_next(Node::new(value));
        }
        true

    }
    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
    pub fn is_empty(&self)-> bool{
        self.head.is_none()
    }

    
}