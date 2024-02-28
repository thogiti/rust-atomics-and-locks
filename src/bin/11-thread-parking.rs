use std::collections::VecDeque;
/// Thread parking example with a que inside a Mutex
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    //define  que with Mutex
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());

    //a newly spawn thread consumes the que
    thread::scope(|s| {
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        //main thread produces the que
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
