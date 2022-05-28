use std::sync::{Arc, Mutex};
use std::thread;

#[allow(dead_code)]
fn main() {
    let counter = Arc::new(Mutex::new(0));

    // Keep track of all the threads we spawn.
    let mut handles = vec![];

    // Create 10 new threads.
    for _ in 0..10 {
        // Create a Mutex for each thread.
        // This acts as a lock, only allowing 1 "person" to use at a time.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            // Blocks the current thread until it is able to do so
            let mut num = counter.lock().unwrap();
            *num += 1
        });
        handles.push(handle);
    }

    for handle in handles {
        // Waits for the associated thread to finish.
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    return
}