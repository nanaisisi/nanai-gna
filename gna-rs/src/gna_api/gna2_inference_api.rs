/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Skeleton for `gna2-inference-api.h` related types and helpers.

/// Request configuration placeholder
#[derive(Debug, Clone)]
pub struct Gna2RequestConfig {
    inner: crate::gna_lib::RequestConfiguration,
}

impl Gna2RequestConfig {
    pub fn id(&self) -> u32 {
        self.inner.config_id
    }
}

pub type InstrumentationConfigId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2AccelerationMode {
    Auto,
    Hardware,
    Software,
}

impl Default for Gna2AccelerationMode {
    fn default() -> Self {
        Gna2AccelerationMode::Auto
    }
}

/// Create a request configuration
pub fn Gna2RequestConfigCreate() -> Gna2RequestConfig {
    Gna2RequestConfig {
        inner: crate::gna_lib::RequestConfiguration::new(),
    }
}

/// Assign an operand buffer
pub fn Gna2RequestConfigSetOperandBuffer(
    cfg: &mut Gna2RequestConfig,
    operand_index: u32,
    addr: crate::common::BaseAddress,
) {
    cfg.inner.set_buffer(operand_index, addr);
}

/// Set instrumentation points to collect for this request configuration
pub fn Gna2RequestConfigSetInstrumentationPoints(
    cfg: &mut Gna2RequestConfig,
    pts: &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint],
) {
    cfg.inner.set_instrumentation_points(pts);
}

/// Set the acceleration mode for this request configuration.
pub fn Gna2RequestConfigSetAccelerationMode(
    cfg: &mut Gna2RequestConfig,
    mode: Gna2AccelerationMode,
) {
    cfg.inner.set_acceleration_mode(mode);
}

/// Get the acceleration mode for this request configuration.
pub fn Gna2RequestConfigGetAccelerationMode(cfg: &Gna2RequestConfig) -> Gna2AccelerationMode {
    cfg.inner.get_acceleration_mode()
}

/// Enqueue request and return request id
pub fn Gna2RequestEnqueue(cfg: &Gna2RequestConfig) -> u32 {
    crate::gna_lib::request::enqueue_request(cfg.inner.clone())
}

/// Wait for request completion; returns true on success
pub fn Gna2RequestWait(request_id: u32, timeout_ms: u32) -> bool {
    crate::gna_lib::request::wait_request(request_id, timeout_ms)
}

/// Retrieve instrumentation results for a finished request (if any). Returns a vector of u64
/// with the same order as the points passed to `Gna2RequestConfigSetInstrumentationPoints`.
pub fn Gna2RequestGetInstrumentationResults(request_id: u32) -> Option<Vec<u64>> {
    crate::gna_lib::request::get_instrumentation_results(request_id)
}

/// Request lifecycle state in the current Rust skeleton.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2RequestState {
    Pending,
    InFlight,
    Completed,
}

/// Query the current lifecycle state for a request.
pub fn Gna2RequestGetState(request_id: u32) -> Option<Gna2RequestState> {
    crate::gna_lib::request::get_request_state(request_id).map(|state| match state {
        crate::gna_lib::request::RequestState::Pending => Gna2RequestState::Pending,
        crate::gna_lib::request::RequestState::InFlight => Gna2RequestState::InFlight,
        crate::gna_lib::request::RequestState::Completed => Gna2RequestState::Completed,
    })
}

/// Returns true when the request is currently in-flight.
pub fn Gna2RequestIsInFlight(request_id: u32) -> bool {
    crate::gna_lib::request::is_request_in_flight(request_id)
}
