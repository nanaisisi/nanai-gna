/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;

/// Stub for HardwareCapabilities

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct HardwareCapabilities;

impl HardwareCapabilities {
    pub fn list() -> Vec<&'static str> {
        Vec::new()
    }

    pub fn get_device_version(&self) -> Gna2DeviceVersion {
        Gna2DeviceVersion(0x30)
    }

    pub fn is_hardware_supported(&self) -> bool {
        true
    }

    pub fn validate_operation_count(&self, _count: u32) {
        // Stubbed validation: accept any operation count.
    }

    pub fn is_operation_supported(
        &self,
        _op: crate::gna_rs::gna_api::types::OperationType,
    ) -> bool {
        true
    }

    pub fn has_feature(&self, _feature: u32) -> bool {
        false
    }
}
