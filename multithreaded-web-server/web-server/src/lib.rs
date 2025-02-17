use std::thread;

pub struct ThreadPool {
    threads: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        println!("number of threads: {size}");
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // add new thread to threads
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
