/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `Request` and request lifecycle management.
use crate::gna_lib::RequestConfiguration;
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Request {
    pub id: u32,
    pub config: RequestConfiguration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestState {
    Pending,
    InFlight,
    Completed,
}

static NEXT_REQUEST_ID: AtomicU32 = AtomicU32::new(1);

impl Request {
    pub fn new(config: RequestConfiguration) -> Self {
        let id = NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        Self { id, config }
    }
}

fn execution_delay_for_mode(mode: crate::gna_api::inference_api::Gna2AccelerationMode) -> Duration {
    match mode {
        crate::gna_api::inference_api::Gna2AccelerationMode::Hardware => Duration::from_millis(5),
        crate::gna_api::inference_api::Gna2AccelerationMode::Auto => Duration::from_millis(10),
        crate::gna_api::inference_api::Gna2AccelerationMode::Software => Duration::from_millis(25),
    }
}

fn instrumentation_results_for_mode(
    mode: crate::gna_api::inference_api::Gna2AccelerationMode,
    points: &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint],
) -> Vec<u64> {
    let (total, stall) = match mode {
        crate::gna_api::inference_api::Gna2AccelerationMode::Hardware => (800u64, 80u64),
        crate::gna_api::inference_api::Gna2AccelerationMode::Auto => (1000u64, 200u64),
        crate::gna_api::inference_api::Gna2AccelerationMode::Software => (1800u64, 500u64),
    };

    points
        .iter()
        .map(|&pt| match pt {
            crate::gna_api::instrumentation_api::Gna2InstrumentationPoint::HwTotalCycles => total,
            crate::gna_api::instrumentation_api::Gna2InstrumentationPoint::HwStallCycles => stall,
            _ => 0u64,
        })
        .collect()
}

// Simple global request queue used by enqueue/wait helpers in this skeleton.

lazy_static::lazy_static! {
    static ref REQUEST_QUEUE: Arc<Mutex<VecDeque<Request>>> = Arc::new(Mutex::new(VecDeque::new()));
    static ref REQUEST_STATES: Arc<Mutex<BTreeMap<u32, RequestState>>> = Arc::new(Mutex::new(BTreeMap::new()));
    // Store simulated instrumentation results for finished requests
    static ref FINISHED_RESULTS: Arc<Mutex<BTreeMap<u32, Vec<u64>>>> = Arc::new(Mutex::new(BTreeMap::new()));
}

pub fn enqueue_request(config: RequestConfiguration) -> u32 {
    let req = Request::new(config);
    let id = req.id;
    REQUEST_QUEUE.lock().unwrap().push_back(req);
    REQUEST_STATES
        .lock()
        .unwrap()
        .insert(id, RequestState::Pending);
    id
}

pub fn process_next_request() {
    let request = {
        let mut q = REQUEST_QUEUE.lock().unwrap();
        q.pop_front()
    };

    if let Some(req) = request {
        REQUEST_STATES
            .lock()
            .unwrap()
            .insert(req.id, RequestState::InFlight);

        let instrumentation_points = req.config.instrumentation_points.clone();
        let mode = req.config.acceleration_mode;

        thread::sleep(execution_delay_for_mode(mode));

        if !instrumentation_points.is_empty() {
            let results = instrumentation_results_for_mode(mode, &instrumentation_points);
            FINISHED_RESULTS.lock().unwrap().insert(req.id, results);
        }

        REQUEST_STATES
            .lock()
            .unwrap()
            .insert(req.id, RequestState::Completed);
    }
}

pub fn wait_request(request_id: u32, timeout_ms: u32) -> bool {
    let start = Instant::now();
    while start.elapsed() <= Duration::from_millis(timeout_ms as u64) {
        if get_request_state(request_id) == Some(RequestState::Completed) {
            return true;
        }
        thread::sleep(Duration::from_millis(1));
    }
    false
}

/// Retrieve instrumentation results for a finished request, if any.
pub fn get_instrumentation_results(request_id: u32) -> Option<Vec<u64>> {
    FINISHED_RESULTS.lock().unwrap().remove(&request_id)
}

/// Query the current lifecycle state for a request.
pub fn get_request_state(request_id: u32) -> Option<RequestState> {
    REQUEST_STATES.lock().unwrap().get(&request_id).copied()
}

/// Returns true when the request is currently in-flight.
pub fn is_request_in_flight(request_id: u32) -> bool {
    get_request_state(request_id) == Some(RequestState::InFlight)
}
