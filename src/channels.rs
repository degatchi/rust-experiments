use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use std::thread;
use std::time;

#[derive(Clone, Debug)]
pub struct Work {
    pub id: u8,
    pub sleep_millis: u16,
    pub done: bool,
}

// How many Work structs to produce.
static MAX_WORK_PRODUCTS: u8 = 5;
static DELAY_IN_BETWEEN_PUBLICATIONS_MILLIS: u8 = 197;

pub fn publish_dummy_work(work_id: u8, tx: Sender<Work>) {
    let curr_work = Work {
        id: work_id,
        sleep_millis: 5000,
        done: false,
    };
    let clone = curr_work.clone();
    println!("PUBLISHER ==> new work: {:?}", clone);
    tx.send(curr_work).unwrap();
    println!("PUBLISHER ==> sent work with ID: {:?}", clone.id);
}

pub fn consume_dummy_work(worker_id: u8, mut work: Work) {
    println!("WORKER {} ==> working (sleeping) for: {:?}", worker_id, work);
    thread::sleep(time::Duration::from_millis(work.sleep_millis as u64));
    work.done = true;
    println!("WORKER {} ==> done with: {:?}", worker_id, work);
}


pub fn channels() {
    // defined a struct to represent some work to do
    // (sleep for an arbitrary amount of time injected at runtime)
    // also some context about the work to be performed (number of message etc.)
    let (tx, rx): (Sender<Work>, Receiver<Work>) = mpsc::channel();

    // define a function to generate and publish some work to do in the channel
    for work_id in 1..(MAX_WORK_PRODUCTS + 1) {
        // send it as a channel message
        let curr_tx = tx.clone();
        publish_dummy_work(work_id, curr_tx);

        thread::sleep(time::Duration::from_millis(
            DELAY_IN_BETWEEN_PUBLICATIONS_MILLIS as u64,
        ));
    }
    println!("MAIN ===> Done publishing...");


    // consume these messages via the channel in separate threads
    let mut handlers = Vec::new();
    let mut msg_counter: u8 = 0;
    for dummy_work in rx.iter() {
        msg_counter += 1;
        println!("MAIN ===> Attempt at spawning for message {}", msg_counter);
        let handler = thread::spawn(move || {
            consume_dummy_work(msg_counter, dummy_work);
        });
        handlers.push(handler);
        println!("MAIN ===> Spawned for message {}", msg_counter);
        

        if msg_counter == MAX_WORK_PRODUCTS {
            println!("MAIN ===> Done - received all the {} messages", msg_counter);
            break;
        }
    }

    // join all the threads to make sure they're all finished
    println!("MAIN ===> Now joining threads...");
    let mut joiner_counter = 0;

    for h in handlers {
        h.join().expect("Cannot wait for thread to finish!");
        joiner_counter += 1;
        println!("MAIN ===> So far joined {} threads", joiner_counter);
    }

    println!("MAIN ===> Done done with everything");
}