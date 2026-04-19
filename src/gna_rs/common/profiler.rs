/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Profiler shim (port of `profiler.h` / `profiler.cpp`).
//
/// Lightweight stub used for tests and later instrumentation.

use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct Profiler {
    start: Option<Instant>,
    accumulated: Duration,
}

impl Profiler {
    pub fn new() -> Self { Self { start: None, accumulated: Duration::ZERO } }

    pub fn start(&mut self) { self.start = Some(Instant::now()) }

    pub fn stop(&mut self) {
        if let Some(s) = self.start {
            self.accumulated += s.elapsed();
            self.start = None;
        }
    }

    pub fn reset(&mut self) { self.start = None; self.accumulated = Duration::ZERO }

    pub fn elapsed(&self) -> Duration { self.accumulated }
}
