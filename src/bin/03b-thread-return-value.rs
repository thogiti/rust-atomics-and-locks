//Thread spawnning with returning a value
use std::thread;

fn main() {
    let num = Vec::from_iter(1..=1000);

    let t = thread::spawn(move || {
        let len = num.len();
        let sum = num.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("Average of the numbers is {:?}", average);
}
