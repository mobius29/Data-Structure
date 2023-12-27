use stack::stack::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::new();
  
    for i in 0..=9 {
        stack.push(i);
    }
  
    while !stack.empty() {
        let top = *stack.top().unwrap();
        stack.pop().unwrap();
  
        println!("{}", top);
    }
  }
  