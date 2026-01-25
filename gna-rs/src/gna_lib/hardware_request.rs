/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for HardwareRequest

#[allow(dead_code)]
pub struct HardwareRequest {
    submitted: bool,
}

impl HardwareRequest {
    pub fn new() -> Self { Self { submitted: false } }

    pub fn submit(&mut self) {
        // lightweight stub — mark as submitted
        self.submitted = true;
    }

    pub fn is_submitted(&self) -> bool { self.submitted }
}
