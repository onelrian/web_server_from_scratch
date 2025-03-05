use std::{sync::{mpsc, Arc, Mutex}, thread};

use crate::job::Job;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}
