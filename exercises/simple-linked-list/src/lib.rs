extern crate core;


pub struct SimpleLinkedList<T> {
    size:usize,
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{
            size:0,
            head:None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        let nd = Node{
            data:_element,
            next: self.head.take(),
        };
        self.head = Some(Box::from(nd));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {return None; }
        let nd = self.head.take().unwrap();
        self.head = nd.next;
        self.size -= 1;
        return Some(nd.data);
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {return None; }
        Some(&self.head.as_ref().unwrap().data)
    }
}


impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut v = vec![];
        let mut rf = self.head.as_ref();
        while rf.is_some() {
            v.push(rf.unwrap().data.clone());
            rf = rf.unwrap().next.as_ref();
        }
        SimpleLinkedList::from(v.as_slice())
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut ns = SimpleLinkedList::new();
        for i in _item {
            ns.push(i.clone());
        }
        ns
    }
}

impl<T:Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = vec![];
        let mut rf = self.head.as_ref();
        while rf.is_some() {
            v.insert(0,rf.unwrap().data.clone());
            rf = rf.unwrap().next.as_ref();
        }
        v
    }
}
