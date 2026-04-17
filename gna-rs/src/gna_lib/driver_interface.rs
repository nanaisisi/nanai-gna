/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::device_api::Gna2DeviceVersion;

/// Minimal Rust port of the GNA `DriverInterface` helper.
#[derive(Debug)]
pub struct DriverInterface {
    device_index: u32,
    opened: bool,
    device_version: Gna2DeviceVersion,
}

impl DriverInterface {
    pub fn new(device_index: u32) -> Self {
        Self {
            device_index,
            opened: false,
            device_version: Self::query(device_index),
        }
    }

    pub fn query(device_index: u32) -> Gna2DeviceVersion {
        // In this stub implementation we return a default device version.
        // A full port would inspect the system device table or driver.
        if device_index == 0 {
            Gna2DeviceVersion(0x30)
        } else {
            Gna2DeviceVersion(0)
        }
    }

    pub fn open(&mut self) -> bool {
        self.opened = true;
        true
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }

    pub fn device_version(&self) -> Gna2DeviceVersion {
        self.device_version
    }
}

#[cfg(test)]
mod tests {
    use super::DriverInterface;
    use crate::gna_api::device_api::Gna2DeviceVersion;

    #[test]
    fn driver_interface_query_returns_default_version_for_index_zero() {
        assert_eq!(DriverInterface::query(0), Gna2DeviceVersion(0x30));
    }

    #[test]
    fn driver_interface_open_marks_instance_as_open() {
        let mut driver = DriverInterface::new(0);
        assert!(!driver.is_open());

        assert!(driver.open());
        assert!(driver.is_open());
    }
}
