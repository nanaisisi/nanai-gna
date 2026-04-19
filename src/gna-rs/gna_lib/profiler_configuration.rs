/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for ProfilerConfiguration

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProfilerConfiguration {
    enabled: bool,
}

impl ProfilerConfiguration {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn enable(&mut self, enable: bool) {
        self.enabled = enable;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
