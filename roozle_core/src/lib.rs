mod search;
mod analysis;
mod buffer;

pub use analysis::*;
pub use buffer::*;
pub use search::*;

use std::sync::OnceLock;
use rayon::{ThreadPool, ThreadPoolBuilder};
use num_cpus;

pub static THREAD_COUNT: OnceLock<usize> = OnceLock::new();
static POOL: OnceLock<ThreadPool> = OnceLock::new();

pub fn set_thread_count(n: usize) {
    if POOL.get().is_some() {
        panic!("Thread pool already initialized");
    }
    if THREAD_COUNT.set(n).is_err() {
        panic!("Thread count already initialized");
    }
}

pub fn get_thread_count() -> Option<&'static usize> {
    THREAD_COUNT.get()
}

fn get_pool() -> &'static ThreadPool {
    POOL.get_or_init(|| {
        let n = THREAD_COUNT.get_or_init(num_cpus::get);
        
        ThreadPoolBuilder::new()
            .num_threads(*n)
            .build()
            .expect("Failed to build thread pool")
    })
}