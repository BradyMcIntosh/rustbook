use std::{
    fmt,
    thread::{self, JoinHandle},
};

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id: id,
            handle: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid pool size")
    }
}

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// Size is the number of workers in the pool.
    ///
    /// # Panics
    ///
    /// Panics if size is zero.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i));
        }

        ThreadPool { workers }
    }

    /// Tries to create a new [`ThreadPool`].
    ///
    /// # Errors
    ///
    /// Returns an error if the pool size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            Err(PoolCreationError)
        } else {
            Ok(ThreadPool::new(size))
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
