/* Tweak is a data structure which allocates nodes on the heap having a pointer to the next and previous
   and the value itself. Tweak itself has a head which points to the first node. Upon reaching (max - 1),
   on insertion from the insert_from_head the last node starts pointing to the head thus completing the cycle
   and upon further insertion the last node is dropped the new element is inserted which becomes the head if
   done from the insert_from_head method and that also has a pointer to the previous node and the previous node
   has a pointer to the new head thus maintaining the cycle.

   * This data structure has been implemented solely for fun
   * It is not production ready
   * There are so many methods yet to implement, the data structure has to be optimized
     and still there would be better options available out there due to the overall design
     of this data structure
*/

use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node<T> {
    #[allow(unused)]
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

#[derive(Debug)]
pub struct Tweak<T> {
    max: usize,
    length: usize,
    head: Option<NonNull<Node<T>>>,
    marker: PhantomData<Node<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

impl<T> Tweak<T> {
    pub fn new(max: usize) -> Self {
        Tweak {
            max,
            length: 0,
            head: None,
            marker: PhantomData,
        }
    }

    pub fn insert_from_head(&mut self, value: T) {
        let mut new = Node::new(value);
        match self.head {
            None => {
                let allocated = Box::into_raw(Box::new(new));
                self.head = NonNull::new(allocated);
                self.length += 1;
            }
            Some(t) => {
                let remaining = self.max - self.length;
                match remaining {
                    0 => {
                        let copy = self.head;
                        if let Some(t) = copy {
                            unsafe {
                                let unwanted = (*t.as_ptr()).prev;
                                let last_raw = unwanted.unwrap().as_ptr();
                                let prev = (*last_raw).prev;
                                new.prev = prev;
                                let _ = Box::from_raw(last_raw);
                                new.next = self.head;
                                let allocated = NonNull::new(Box::into_raw(Box::new(new)));
                                (*self.head.unwrap().as_ptr()).prev = allocated;
                                (*prev.unwrap().as_ptr()).next = allocated;
                                self.head = allocated;
                            }
                        }
                    }
                    1 => {
                        new.next = self.head;
                        let mut copy = new.next;
                        let iteration = self.max - 2;
                        for _ in 0..iteration {
                            if let Some(a) = copy {
                                let next = unsafe { (*a.as_ptr()).next };
                                copy = next;
                            }
                        }
                        new.prev = copy;
                        let allocated = NonNull::new(Box::into_raw(Box::new(new)));
                        unsafe {
                            (*t.as_ptr()).prev = allocated;
                        }
                        let p = copy;
                        unsafe {
                            (*p.unwrap().as_ptr()).next = allocated;
                        }
                        self.head = allocated;
                        self.length += 1;
                    }
                    _ => {
                        new.next = self.head;
                        let allocated = NonNull::new(Box::into_raw(Box::new(new)));
                        unsafe {
                            (*t.as_ptr()).prev = allocated;
                        }
                        self.head = allocated;
                        self.length += 1;
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for Tweak<T> {
    fn drop(&mut self) {
        let mut check = self.head;
        match check {
            None => return,
            Some(_) => {
                if self.length == self.max {
                    let mut last = self.head;
                    for _ in 0..(self.length - 1) {
                        last = unsafe { (*last.unwrap().as_ptr()).next };
                    }
                    unsafe {
                        (*last.unwrap().as_ptr()).next = None;
                    }
                    while let Some(a) = check {
                        let to_be_dropped = unsafe { Box::from_raw(a.as_ptr()) };
                        check = to_be_dropped.next;
                        drop(to_be_dropped);
                    }
                } else {
                    while let Some(a) = check {
                        let to_be_dropped = unsafe { Box::from_raw(a.as_ptr()) };
                        check = to_be_dropped.next;
                        drop(to_be_dropped);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut new = Tweak::new(4);
        new.insert_from_head(1);
        new.insert_from_head(2);
        new.insert_from_head(3);
        println!("{}", new.len());
        new.insert_from_head(4);
        new.insert_from_head(5);
        println!("{}", new.len());
        assert_eq!(4, new.len());
    }
}
