use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
fn main() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    let non_empty = Condvar::new();

    thread::scope(|s| {
        //consuming thtread
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    q = non_empty.wait(q).unwrap();
                }
            };
            drop(q);
            dbg!(item);
        });

        //producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            non_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
