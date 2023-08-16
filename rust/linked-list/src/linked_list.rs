use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: NextNode<T>,
}

type NextNode<T> = Option<NonNull<Node<T>>>;

pub struct LinkedList<T> {
    size: u32,
    head: NextNode<T>,
    tail: NextNode<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn make_node(value: T) -> NextNode<T> {
        let node = Box::new(Node::new(value));
        NonNull::new(Box::into_raw(node))
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = LinkedList::make_node(value);
        unsafe {
            let new_node_ptr = new_node.unwrap().as_ptr();
            (*new_node_ptr).next = self.head;
        }

        if self.head.is_none() {
            self.tail = new_node;
        }

        self.head = new_node;
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = LinkedList::make_node(value);

        if self.tail.is_none() {
            self.head = new_node;
        } else {
            unsafe {
                let tail_ptr = self.tail.unwrap().as_ptr();
                (*tail_ptr).next = new_node;
            }
        }

        self.tail = new_node;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Result<(), &str> {
        match self.head {
            Some(head_ptr) => {
                unsafe {
                    self.head = (*head_ptr.as_ptr()).next;
                }
                Ok(())
            }
            None => Err("list is empty!"),
        }
    }

    pub fn pop_back(&mut self) -> Result<(), &str> {
        match self.tail {
            Some(tail_ptr) => {
                todo!();
            },
            None => Err("list is empty!"),
        }
    }
}
