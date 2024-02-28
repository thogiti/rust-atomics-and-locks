/// Use a thread closure to move the id number into a thread
use std::thread;

fn main() {
    let num = vec![1, 2, 3];

    // a thread closure demo
    thread::spawn(move || {
        for n in &num {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}
