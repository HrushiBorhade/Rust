use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new ThreadPool with the given size.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// This function panics if the `size` is zero.
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("ThreadPool size must be greater than zero");
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes a job in the thread pool.
    ///
    /// This method sends a job to the thread pool to be executed by one of the worker threads.
    ///
    /// # Error Handling
    ///
    /// The `send` function will return an error if the receiver end of the channel is dropped,
    /// indicating the thread pool has been shut down. We handle this case gracefully by
    /// logging an error and not panicking.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            // Error handling for the send operation. If the sender is None or the receiver is dropped,
            // this will log an error without panicking.
            if let Err(e) = sender.send(job) {
                eprintln!("Error sending job to thread pool: {}", e);
            }
        } else {
            eprintln!("Sender is not available. The thread pool might have been dropped.");
        }
    }
}

impl Drop for ThreadPool {
    /// Cleans up and shuts down the thread pool when it goes out of scope.
    ///
    /// # Proper shutdown
    ///
    /// We ensure each worker thread is properly joined. If any worker thread has been
    /// dropped (due to a panic or other errors), the `join` will return an error, which
    /// we handle by logging the issue.
    fn drop(&mut self) {
        // Dropping the sender to signal to workers that they should stop.
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("Error joining worker {}: {:?}", worker.id, e);
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker thread and starts it to listen for jobs.
    ///
    /// This method wraps the worker thread logic in a `loop` that waits for jobs
    /// and executes them. The worker listens to the receiver channel for incoming jobs.
    ///
    /// # Error Handling
    ///
    /// The `recv` method on the receiver channel will block until a job is received.
    /// If the receiver is disconnected (i.e., the thread pool is dropped), the worker
    /// should exit gracefully. We handle the error case by logging that the worker
    /// was disconnected and stopping the loop.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .unwrap_or_else(|e| {
                    eprintln!("Worker {}: failed to acquire lock: {}", id, e);
                    std::process::exit(1); // Exit if locking fails.
                })
                .recv();

            match job {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(e) => {
                    eprintln!("Worker {} disconnected; shutting down. Error: {}", id, e);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
