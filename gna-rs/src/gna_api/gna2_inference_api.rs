//! Skeleton for `gna2-inference-api.h` related types and helpers.

/// Request configuration placeholder
#[derive(Debug, Clone)]
pub struct Gna2RequestConfig {
    inner: crate::gna_lib::RequestConfiguration,
}

impl Gna2RequestConfig {
    pub fn id(&self) -> u32 { self.inner.config_id }
}

pub type InstrumentationConfigId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2AccelerationMode {
    Auto,
    Hardware,
    Software,
}

impl Default for Gna2AccelerationMode { fn default() -> Self { Gna2AccelerationMode::Auto } }

/// Create a request configuration
pub fn Gna2RequestConfigCreate() -> Gna2RequestConfig {
    Gna2RequestConfig { inner: crate::gna_lib::RequestConfiguration::new() }
}

/// Assign an operand buffer
pub fn Gna2RequestConfigSetOperandBuffer(cfg: &mut Gna2RequestConfig, operand_index: u32, addr: crate::common::BaseAddress) {
    cfg.inner.set_buffer(operand_index, addr);
}

/// Enqueue request and return request id
pub fn Gna2RequestEnqueue(cfg: &Gna2RequestConfig) -> u32 {
    crate::gna_lib::request::enqueue_request(cfg.inner.clone())
}

/// Wait for request completion; returns true on success
pub fn Gna2RequestWait(request_id: u32, timeout_ms: u32) -> bool {
    crate::gna_lib::request::wait_request(request_id, timeout_ms)
}
