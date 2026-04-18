/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::device_api::Gna2DeviceVersion;

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
}
