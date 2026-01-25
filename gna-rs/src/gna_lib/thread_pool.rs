//! Stub for ThreadPool

#[allow(dead_code)]
pub struct ThreadPool;

impl ThreadPool {
    pub fn spawn<F: FnOnce()+Send+'static>(&self, _f: F) { /* TODO */ }
}
