/// Main function finishes before the myfunc get its id.
use std::thread;

fn myfunc() {
    print!("Hello from new thread.\n");

    //Get the thread id and print it
    let id = thread::current().id();
    print!("My thread id is {id:?} \n");
}

fn main() {
    //Spawn two threads and print their id
    thread::spawn(myfunc);
    thread::spawn(myfunc);

    print!("Hello from main thread.\n");
}
