/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for ThreadPool

#[allow(dead_code)]
pub struct ThreadPool;

impl ThreadPool {
    pub fn spawn<F: FnOnce()+Send+'static>(&self, _f: F) { /* TODO */ }
}
