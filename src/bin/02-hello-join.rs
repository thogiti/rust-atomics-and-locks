//Main function waits until the threads are finished executing i.e. get their ids
use std::thread;

fn main() {
    let t1 = thread::spawn(myfunc);
    let t2 = thread::spawn(myfunc);

    println!("Hello main from thread.");

    //the join method waits until the thread has finished executing
    t1.join().unwrap();
    t2.join().unwrap();
}

fn myfunc() {
    println!("Hello from new thread.");
    let id = thread::current().id();

    println!("My thread id is {id:?}");
}
