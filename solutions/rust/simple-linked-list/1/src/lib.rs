
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }
        len
    }

    pub fn push(&mut self, element: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node { value: element, next: old_head }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let current = self.head.take();
        if let Some(node) = current {
            self.head = node.next;
            Some(node.value)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut prev = None;
        let mut curr = self.head;
        while let Some(mut node) = curr {
            let tmp = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = tmp;
        }
        SimpleLinkedList {
            head: prev,
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}


impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        let mut reversed = linked_list.rev();
        while let Some(element) = reversed.pop() {
            vec.push(element);
        }
        vec
    }
}
