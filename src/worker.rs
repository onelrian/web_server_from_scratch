use std::thread;

pub struct Worker {
    id: usize,
    thread : thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        Worker{
            id,
            thread
        }
    }
}