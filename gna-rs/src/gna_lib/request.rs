//! Skeleton for `Request` and request lifecycle management.

use crate::gna_lib::RequestConfiguration;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::thread;

#[derive(Debug)]
pub struct Request {
    pub id: u32,
    pub config: RequestConfiguration,
}

static NEXT_REQUEST_ID: AtomicU32 = AtomicU32::new(1);

impl Request {
    pub fn new(config: RequestConfiguration) -> Self {
        let id = NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        Self { id, config }
    }
}

// Simple global request queue used by enqueue/wait helpers in this skeleton.

lazy_static::lazy_static! {
    static ref REQUEST_QUEUE: Arc<Mutex<VecDeque<Request>>> = Arc::new(Mutex::new(VecDeque::new()));
}

pub fn enqueue_request(config: RequestConfiguration) -> u32 {
    let req = Request::new(config);
    let id = req.id;
    REQUEST_QUEUE.lock().unwrap().push_back(req);
    id
}

pub fn wait_request(request_id: u32, timeout_ms: u32) -> bool {
    // Simulate processing: pop from queue when id matches, or time out
    let start = Instant::now();
    loop {
        {
            let mut q = REQUEST_QUEUE.lock().unwrap();
            if let Some(pos) = q.iter().position(|r| r.id == request_id) {
                // simulate immediate processing
                q.remove(pos);
                return true;
            }
        }
        if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
            return false;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
