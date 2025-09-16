use std::{marker::PhantomData, ptr::NonNull};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

type Link<T> = Option<NonNull<Node<T>>>;

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new_front = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old_front) = self.front {
                (*old_front.as_ptr()).front = Some(new_front);
                (*new_front.as_ptr()).back = Some(old_front);
            } else {
                self.back = Some(new_front);
            }
            self.front = Some(new_front);
            self.len += 1;
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|front| {
                let old_front = front.as_ptr();
                self.front = (*old_front).back;
                if let Some(new_front) = self.front {
                    (*new_front.as_ptr()).front = None;
                } else {
                    self.back = None;
                }
                self.len -= 1;
                Box::from_raw(old_front).elem // Bring back the Box very late?
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|front| &(*front.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|front| &mut (*front.as_ptr()).elem) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|front| unsafe {
                self.len -= 1;
                self.front = (*front.as_ptr()).back;
                &(*front.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|back| unsafe {
                self.len -= 1;
                self.back = (*back.as_ptr()).front;
                &(*back.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
