#![cfg(feature = "rust_backend")]

use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Simple software-side worker to emulate GNA processing for the `rust_backend` feature.
/// This is a lightweight stub that mimics request latency and updates the same
/// statistics used by the `load_test` example.
pub fn gna_worker_rust(
    thread_id: u32,
    stop: Arc<AtomicBool>,
    ops: Arc<AtomicU64>,
    latency_sum: Arc<AtomicU64>,
    latency_min: Arc<AtomicU64>,
    latency_max: Arc<AtomicU64>,
    _device_open_close: bool,
    _timeout_ms: u32,
) {
    // Simple loop that waits a small time to simulate work and updates stats
    while !stop.load(Ordering::Relaxed) {
        let t0 = Instant::now();
        // Simulate some compute+memory work
        thread::sleep(Duration::from_millis(1));
        let dur = Instant::now().duration_since(t0);
        let us = dur.as_micros() as u64;

        ops.fetch_add(1, Ordering::Relaxed);
        latency_sum.fetch_add(us, Ordering::Relaxed);

        // update min
        let mut cur_min = latency_min.load(Ordering::Relaxed);
        while us < cur_min {
            match latency_min.compare_exchange(cur_min, us, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(old) => cur_min = old,
            }
        }
        // update max
        let mut cur_max = latency_max.load(Ordering::Relaxed);
        while us > cur_max {
            match latency_max.compare_exchange(cur_max, us, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(old) => cur_max = old,
            }
        }
    }
}
