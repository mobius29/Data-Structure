use deque::deque::Deque;

fn main() {
    let mut deque: Deque<i32> = Deque::new();

    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_front(4);
    deque.push_front(5);
    deque.push_front(6);

    while !deque.empty() {
        println!("{}", deque.front().unwrap());
        deque.pop_front().ok();
    }
}
