pub struct Stack<T> {
  head: NextNode<T>,
  size: usize,
}

struct Node<T> {
  value: T,
  next_node: NextNode<T>,
}

type NextNode<T> = Option<Box<Node<T>>>;

impl<T> Stack<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      size: 0,
    }
  }

  pub fn push(&mut self, value: T) {
    let new_node = Box::new(Node {
      value,
      next_node: self.head.take(),
    });

    self.size += 1;
    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Result<(), &str> {
    match self.head.take() {
      Some(node) => {
        self.head = node.next_node;
        self.size -= 1;

        Ok(())
      },
      None => Err("Cannot pop this stack because stack is empty."),
    }
  }

  pub fn top(&mut self) -> Result<&T, &str> {
    match self.head.as_ref() {
      Some(node) => Ok(&node.value),
      None => Err("Cannot get value of this stack because stack is empty."),
    }
  }

  pub fn top_mut(&mut self) -> Result<&mut T, &str> {
    match self.head.as_mut() {
      Some(node) => Ok(&mut node.value),
      None => Err("Cannot get value of this stack because stack is empty."),
    }
  }

  pub fn empty(&self) -> bool {
    self.head.is_none()
  }

  pub fn clear(&mut self) -> () {
    while !self.empty() {
      self.pop().unwrap();
    }
  }
}

