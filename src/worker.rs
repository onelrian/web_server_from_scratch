use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::job::Job;

pub struct Worker {
    id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a Job ; Executing.");
                    job();
                }

                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break; 
                }
            }
        });

        Worker { id, thread }
    }

    pub fn get_worker_id(&self) -> usize {
        self.id
    }

    pub fn set_worker_id(&mut self, id: usize) {
        self.id = id
    }
}
