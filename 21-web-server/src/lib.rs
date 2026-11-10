use std::fmt;

pub struct ThreadPool;

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
    /// Size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// Panics if size is zero.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
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
