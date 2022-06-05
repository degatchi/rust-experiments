use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_one() {

    // we need to unlock the mutex to be able to see the value that has been updated inside
    let my_mutex_v1 = Mutex::new(41);
    let mut mutex_val_v1 = my_mutex_v1.lock().unwrap();
    // we attempt to modify the value, but the lock is not released
    *mutex_val_v1 = 42;
    println!("my_mutex_v1: {:?}\n", my_mutex_v1);

    
    // -----------------------------------------------------------------------------
    // scopes and how a mutex is unlocked automatically
    // e.g. after a block or a thread is spawned
    // -----------------------------------------------------------------------------
    // a mutex is unlocked when the `.lock()` goes out of scope
    // e.g. when going outside of a block `{...}` back in the `main` function
    let my_mutex_v2 = Mutex::new(41);
    {
        let mut mutex_val_v2 = my_mutex_v2.lock().unwrap();
        *mutex_val_v2 = 42; // modify value attempt
    } // once outside the block, the lock is released automatically
    println!("my_mutex_v2: {:?}\n", my_mutex_v2);


    // -----------------------------------------------------------------------------
    // Issues with threads and moving a mutex into them
    // losing access to it from outside the thread
    // (this is the usual ownership/borrowing model in Rust)
    // -----------------------------------------------------------------------------
    // what if we want to send this mutex to a thread?
    // there are issues with borrowing and ownership due to the `move` keyword
    // because the `main` function has given away the mutex to the spawned thread

    // let my_mutex_v3 = Mutex::new(41);
    // thread::spawn(move || {
    //     let mut my_mutex_v3 = my_mutex_v3.lock().unwrap();
    //     *mutex_val_v3 = 42;
    // });
    // println!("my_mutex_v3: {:?}", my_mutex_v3);


    // -----------------------------------------------------------------------------
    // using an atomically reference counted to work around the Rust ownership model
    // so multiple owners (threads) can reference to the same resource in a thread safe way
    // -----------------------------------------------------------------------------
    let my_arc_mutex_v4 = Arc::new(Mutex::new(41));
    let my_arc_mutex_v4_clone = Arc::clone(&my_arc_mutex_v4);
    println!("Mutex value before: {:?}", my_arc_mutex_v4);

    let handler = thread::spawn(move || {
        println!("reference count (before thread joins main): {:?}", Arc::strong_count(&my_arc_mutex_v4_clone));
        let mut mutex_val_v4 = my_arc_mutex_v4_clone.lock().unwrap();
        *mutex_val_v4 = 42;
    });

    handler.join().unwrap();
    println!("Mutex value after: {:?}", my_arc_mutex_v4);
    println!("reference count (after thread died): {:?}", Arc::strong_count(&my_arc_mutex_v4));
}