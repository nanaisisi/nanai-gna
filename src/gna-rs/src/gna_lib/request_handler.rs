use crate::gna_lib::request;
use crate::gna_lib::request::Request;
/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::request::{enqueue_request, get_request_state, wait_request};
use crate::gna_lib::thread_pool::ThreadPool;

/// Simplified Rust port of the GNA `RequestHandler` helper.
#[derive(Debug)]
pub struct RequestHandler {
    thread_pool: ThreadPool,
}

impl RequestHandler {
    pub fn new() -> Self {
        Self {
            thread_pool: ThreadPool {},
        }
    }

    pub fn get_number_of_threads(&self) -> u32 {
        1
    }

    pub fn change_number_of_threads(&mut self, _thread_count: u32) {
        // Threading is simplified in this skeleton.
    }

    pub fn enqueue(&self, request: Request) -> u32 {
        let request_id = enqueue_request(request.config);
        self.thread_pool.spawn(move || {
            request::process_next_request();
        });
        request_id
    }

    pub fn wait_for(&self, request_id: u32, timeout_ms: u32) -> bool {
        wait_request(request_id, timeout_ms)
    }

    pub fn stop_requests(&self) {
        // No-op in simplified request handling.
    }

    pub fn has_request(&self, request_id: u32) -> bool {
        get_request_state(request_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::RequestHandler;
    use crate::gna_api::inference_api::Gna2AccelerationMode;
    use crate::gna_api::instrumentation_api::Gna2InstrumentationPoint;
    use crate::gna_lib::request::get_instrumentation_results;
    use crate::gna_lib::{Request, RequestConfiguration};

    #[test]
    fn request_handler_can_enqueue_and_wait() {
        let handler = RequestHandler::new();
        let config = RequestConfiguration::new();
        let request = Request::new(config);
        let request_id = handler.enqueue(request);

        assert!(handler.has_request(request_id));
        assert!(handler.wait_for(request_id, 100));
    }

    #[test]
    fn request_handler_runs_worker_thread_and_populates_instrumentation_results() {
        let handler = RequestHandler::new();
        let mut config = RequestConfiguration::new();
        config.set_instrumentation_points(&[
            Gna2InstrumentationPoint::HwTotalCycles,
            Gna2InstrumentationPoint::HwStallCycles,
        ]);
        config.set_acceleration_mode(Gna2AccelerationMode::Hardware);

        let request = Request::new(config);
        let request_id = handler.enqueue(request);

        assert!(handler.wait_for(request_id, 200));
        let results = get_instrumentation_results(request_id);
        assert!(results.is_some());
        assert_eq!(results.unwrap().len(), 2);
    }
}
