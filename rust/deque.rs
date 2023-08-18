use std::ptr::NonNull;

struct Node<T> {
  value: T,
  prev: Next<T>,
  next: Next<T>,
}

impl<T> Node<T> {
  pub fn new(value: T, prev: Next<T>, next: Next<T>) -> Self {
      Self { value, prev, next }
  }
}

type Next<T> = Option<NonNull<Node<T>>>;

struct Deque<T> {
  front: Next<T>,
  rear: Next<T>,
  size: usize,
}

impl<T> Deque<T> {
  pub fn new() -> Self {
      Self {
          front: None,
          rear: None,
          size: 0,
      }
  }

  pub fn push_front(&mut self, value: T) {
      let new_node = Box::new(Node::new(value, None, self.front));
      let new_node_ptr = NonNull::new(Box::into_raw(new_node));

      match self.front {
          None => self.rear = new_node_ptr,
          Some(front_ptr) => unsafe {
              (*front_ptr.as_ptr()).prev = new_node_ptr;
          },
      };

      self.front = new_node_ptr;
      self.size += 1;
  }

  pub fn push_back(&mut self, value: T) {
      let new_node = Box::new(Node::new(value, self.rear, None));
      let new_node_ptr = NonNull::new(Box::into_raw(new_node));

      match self.rear {
          None => self.front = new_node_ptr,
          Some(rear_ptr) => unsafe {
              (*rear_ptr.as_ptr()).next = new_node_ptr;
          },
      };

      self.rear = new_node_ptr;
      self.size += 1;
  }

  pub fn pop_front(&mut self) -> Result<(), String> {
      match self.front {
          None => Err("This Deque is empty!".to_string()),
          Some(front_ptr) => unsafe {
              let next_ptr = (*front_ptr.as_ptr()).next;
              match next_ptr {
                  None => {
                      self.rear = None;
                  }
                  Some(next) => {
                      (*next.as_ptr()).prev = None;
                  }
              }
              self.front = next_ptr;
              self.size -= 1;

              Ok(())
          },
      }
  }

  pub fn pop_back(&mut self) -> Result<(), String> {
      match self.rear {
          None => Err("This Deque is empty!".to_string()),
          Some(rear_ptr) => unsafe {
              let prev_ptr = (*rear_ptr.as_ptr()).prev;
              match prev_ptr {
                  None => {
                      self.front = None;
                  },
                  Some(prev) => {
                      (*prev.as_ptr()).next = None;
                  }
              }
              self.rear = prev_ptr;
              self.size -= 1;

              Ok(())
          },
      }
  }

  pub fn size(&self) -> usize {
      self.size
  }

  pub fn empty(&self) -> bool {
      self.size == 0
  }

  pub fn front(&self) -> Result<&T, String> {
      match self.front.as_ref() {
          None => Err("This Deque is empty!".to_string()),
          Some(front_ptr) => unsafe { Ok(&(*front_ptr.as_ptr()).value) },
      }
  }

  pub fn back(&self) -> Result<&T, String> {
      match self.rear.as_ref() {
          None => Err("This Deque is empty!".to_string()),
          Some(rear_ptr) => unsafe { Ok(&(*rear_ptr.as_ptr()).value) },
      }
  }
}