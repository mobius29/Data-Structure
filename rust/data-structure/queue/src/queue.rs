use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Next<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Box<Node<T>> {
        Box::new(Node { value, next: None })
    }
}

type Next<T> = Option<NonNull<Node<T>>>;

pub struct Queue<T> {
    front: Next<T>,
    back: Next<T>,
    size: u32,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            back: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    fn get_node_ptr(node: Box<Node<T>>) -> Next<T> {
        NonNull::new(Box::into_raw(node))
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Node::new(value);
        let new_node_ptr = Queue::get_node_ptr(new_node);

        match self.back {
            Some(back_ptr) => unsafe {
                (*back_ptr.as_ptr()).next = new_node_ptr;
                self.back = new_node_ptr;
            },
            None => self.front = new_node_ptr,
        };

        self.back = new_node_ptr;
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Result<(), &str> {
        match self.front {
            Some(front_ptr) => {
                unsafe {
                    self.front = (*front_ptr.as_ptr()).next;
                }

                self.size -= 1;
                if self.size == 0 {
                    self.back = None;
                }
                Ok(())
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn front(&self) -> Result<&T, &str> {
        match self.front {
            Some(front_ptr) => {
                let return_value = unsafe { &(*front_ptr.as_ptr()).value };
                Ok(return_value)
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn front_mut(&self) -> Result<&mut T, &str> {
        match self.front {
            Some(front_ptr) => {
                let return_value = unsafe { &mut (*front_ptr.as_ptr()).value };
                Ok(return_value)
            }
            None => Err("queue is empty!"),
        }
    }

    pub fn clear(&mut self) {
        while !self.empty() {
            self.dequeue().ok();
        }
    }
}
