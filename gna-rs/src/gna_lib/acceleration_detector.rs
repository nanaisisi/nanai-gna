/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for AccelerationDetector (ported from original C++)

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AccelerationDetector;

impl AccelerationDetector {
    /// Detects available kernels / accelerations (stub).
    pub fn detect() -> Vec<&'static str> {
        Vec::new()
    }
}
