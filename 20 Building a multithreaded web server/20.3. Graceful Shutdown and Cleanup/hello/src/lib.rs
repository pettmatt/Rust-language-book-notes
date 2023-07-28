use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }
    // Don't mind the styling. This is cleaner and easier to read for me.
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static, 
        {
            let job = Box::new(f);
            
            // Compared to previous 'send' execution we need to process the Some to access the sender and use send method.
            // self.sender.send(job).unwrap();
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // We call take on the Option value to move thread out of worker.
            if let Some(thread) = worker.thread.take() {
                // If accessing to join fails we need to call unwrap which will just panic and exit ungracefully.
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // let job = receiver.lock().unwrap().recv().unwrap();
            let message = receiver.lock().unwrap().recv();

            // So cool match... don't ask why. It just is.
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}