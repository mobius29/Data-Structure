use queue::queue::Queue;

fn main() {
    let mut queue: Queue<i32> = Queue::new();

    for i in 0..10 {
        queue.enqueue(i);
    }

    while !queue.empty() {
        let front = queue.front().unwrap();
        print!("{} ", front);

        queue.dequeue().ok();
    }
}
