/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for Logger

#[allow(dead_code)]
pub struct Logger;

impl Logger {
    pub fn log(&self, msg: &str) {
        eprintln!("[GNA-RS LOG] {}", msg);
    }
}
