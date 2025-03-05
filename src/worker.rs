use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::job::{self, Job};

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a Job ; Executing.");
                job();
            }
        });

        Worker { id, thread }
    }
}
