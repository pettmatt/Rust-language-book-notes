use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHangle<()>>
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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {

        }

        ThreadPool { threads }
    }
    // Don't mind the styling. This is cleaner and easier to read for me.
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static, 
        {
        }
}