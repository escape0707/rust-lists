use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        if let Some(old_head) = self.head.take() {
            old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
            new_head.borrow_mut().next = Some(old_head);
            self.head = Some(new_head);
        } else {
            self.head = Some(Rc::clone(&new_head));
            self.tail = Some(new_head);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.tail = None;
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<'_, T>> {
        self.head
            .as_ref()
            .map(|head| Ref::map(head.borrow(), |head| &head.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<'_, T>> {
        self.head
            .as_ref()
            .map(|head| RefMut::map(head.borrow_mut(), |head| &mut head.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
            new_tail.borrow_mut().prev = Some(old_tail);
            self.tail = Some(new_tail);
        } else {
            self.head = Some(Rc::clone(&new_tail));
            self.tail = Some(new_tail);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            } else {
                self.head = None;
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<'_, T>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<'_, T>> {
        self.tail
            .as_ref()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
