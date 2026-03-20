use crate::structures::Node::Node;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

    pub fn contains(&self, value: &T) -> bool
    where T: PartialEq
    {
    let mut current = self.head.as_deref();
    while let Some(node) = current {
        if node.get_value() == value {
            return true;
        }
        current = node.get_next().map(|boxed| boxed.as_ref());
    }
    false
    }

    pub fn remove(&mut self, value: &T) -> bool 
    where T: PartialEq
    {
    let mut current = &mut self.head;

    loop {
        match current {
            None => return false,
            Some(node) if node.get_value() == value => {
                *current = node.take_next();
                return true;
            }
            Some(node) => {
                current = node.next_mut();
            }
        }
    }
    }
    pub fn iterator(&self)->Iter<T>{
        Iter{next:self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.get_next().map(|n| n.as_ref());
            node.get_value()
        })
    }
}
