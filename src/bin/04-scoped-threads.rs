/// Scoped threads demo
use std::thread;
fn main() {
    let num = vec![1, 2, 3];

    //threads are within the scope s; threads are guaranteed not live out of the scope s
    thread::scope(|s| {
        s.spawn(|| {
            println!("Length of num vector is {:?}", num.len());
        });
        s.spawn(|| {
            for n in &num {
                println!("{n}");
            }
        });
    });
}
