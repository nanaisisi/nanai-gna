/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_lib::device::Device;

/// Simplified Rust port of the GNA `DeviceManager` helper.
#[derive(Debug, Default)]
pub struct DeviceManager;

impl DeviceManager {
    /// Enumerate available devices in the current system stub.
    pub fn enumerate() -> Vec<Device> {
        vec![Device::new(Gna2DeviceVersion(0x30))]
    }

    /// Return the number of enumerated devices.
    pub fn device_count() -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::DeviceManager;

    #[test]
    fn device_manager_enumerate_returns_at_least_one_device() {
        let devices = DeviceManager::enumerate();
        assert_eq!(devices.len(), 1);
    }

    #[test]
    fn device_manager_device_count_returns_one() {
        assert_eq!(DeviceManager::device_count(), 1);
    }
}
