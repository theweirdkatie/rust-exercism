use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        !self.head.is_some()
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current_node = &self.head;
        while let Some(next_node) = current_node {
            length += 1;
            current_node = &next_node.next;
        }
        length
    }

    pub fn push(&mut self, _element: T) {
        let node = Box::new(Node::new(_element, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head_node) = self.head.take() {
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head_node) = &self.head {
            Some(&head_node.data)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed_list = SimpleLinkedList::new();
        while let Some(node) = self.pop() {
            reversed_list.push(node);
        }
        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in _iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result_vec = Vec::new();
        while let Some(node) = _linked_list.pop() {
            result_vec.insert(0, node);
        }
        result_vec
    }
}
