use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;



#[allow(dead_code)]
pub fn example_1() {
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


pub fn example_2() {
    let steps = Arc::new(Mutex::new(5));

    let thread = {
        let steps = steps.clone();
        thread::spawn(move || {
            while *steps.lock().unwrap() > 0 {
                sleep(Duration::from_secs(1));
                println!("Thread step {}", steps.lock().unwrap());
                *steps.lock().unwrap() -= 1;
            }
            // "Goodbye!"
        })
    };
    println!("Spawned a thread with a step count of {}!", steps.lock().unwrap());
    sleep(Duration::from_secs(2));
    println!("Slept 2 secs. Now joining thread...");
    let result = thread.join().unwrap();
    println!("Thread returned with {:?}", result);
}