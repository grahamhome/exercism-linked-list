use std::iter::FromIterator;

mod tests;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.as_deref();
        let mut count = 0;
        while let Some(node) = current {
            current = node.next.as_deref();
            count += 1;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            value: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        return if self.is_empty() {
            None
        } else {
            let node = self.head.take().unwrap();
            self.head = node.next;
            Some(node.value)
        };
    }

    pub fn peek(&self) -> Option<&T> {
        return if self.is_empty() {
            None
        } else {
            Some(&self.head.as_deref().unwrap().value)
        };
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while !self.is_empty() {
            rev_list.push(self.pop().unwrap())
        }
        rev_list
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut new_list = SimpleLinkedList::new();
        let mut iter = _iter.into_iter();
        while let Some(value) = iter.next() {
            new_list.push(value)
        }
        new_list
    }
}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        _linked_list
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}
