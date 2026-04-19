/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_lib::hardware_request::HardwareRequest;

/// Minimal Rust port of the GNA `DriverInterface` helper.
#[derive(Debug, Clone)]
pub struct DriverInterface {
    device_index: u32,
    opened: bool,
    device_version: Gna2DeviceVersion,
}

#[derive(Debug, Clone)]
pub struct DriverPerf {
    pub preprocessing: u32,
    pub processing: u32,
    pub device_request_completed: u32,
    pub completion: u32,
}

#[derive(Debug, Clone)]
pub struct HardwarePerf {
    pub total: u32,
    pub stall: u32,
}

#[derive(Debug, Clone)]
pub struct DriverSubmissionResult {
    pub status: u32,
    pub driver_perf: DriverPerf,
    pub hardware_perf: HardwarePerf,
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

    pub fn submit_request(&self, hardware_request: &HardwareRequest) -> DriverSubmissionResult {
        let status = if hardware_request.submit_ready { 0 } else { 1 };
        DriverSubmissionResult {
            status,
            driver_perf: DriverPerf {
                preprocessing: 0,
                processing: 0,
                device_request_completed: 0,
                completion: 0,
            },
            hardware_perf: HardwarePerf { total: 0, stall: 0 },
        }
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

    #[test]
    fn driver_interface_submit_request_returns_error_when_not_ready() {
        let driver = DriverInterface::new(0);
        let config = crate::gna_lib::request_configuration::RequestConfiguration::new();
        let req = crate::gna_lib::hardware_request::HardwareRequest::new(config);

        let result = driver.submit_request(&req);
        assert_ne!(result.status, 0);
    }

    #[test]
    fn driver_interface_submit_request_returns_success_when_ready() {
        let driver = DriverInterface::new(0);
        let mut config = crate::gna_lib::request_configuration::RequestConfiguration::new();
        let mut req = crate::gna_lib::hardware_request::HardwareRequest::new(config);
        req.submit_ready = true;

        let result = driver.submit_request(&req);
        assert_eq!(result.status, 0);
    }
}
