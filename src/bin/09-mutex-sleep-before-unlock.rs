/// Mutex with sleep before unlock
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut n = n.lock().unwrap();
                for _ in 0..1000 {
                    *n += 1;
                }
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 10000)
}
