/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for DeviceManager (ported from original C++)

use crate::gna_lib::device::Device;

#[allow(dead_code)]
pub struct DeviceManager;

impl DeviceManager {
    pub fn enumerate() -> Vec<Device> { Vec::new() }
}
