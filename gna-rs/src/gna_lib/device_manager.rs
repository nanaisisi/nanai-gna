//! Stub for DeviceManager (ported from original C++)

use crate::gna_lib::device::Device;

#[allow(dead_code)]
pub struct DeviceManager;

impl DeviceManager {
    pub fn enumerate() -> Vec<Device> { Vec::new() }
}
