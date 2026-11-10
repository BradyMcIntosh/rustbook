pub struct ThreadPool;

// TODO: pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}
impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// Size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// Panics if size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
