/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `RequestConfiguration`.

use crate::common::BaseAddress;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone)]
pub struct RequestConfiguration {
    pub buffers: BTreeMap<u32, BaseAddress>,
    pub timeout_ms: u32,
    pub config_id: u32,
    /// Optional instrumentation points to collect for this request
    pub instrumentation_points: Vec<crate::gna_api::instrumentation_api::Gna2InstrumentationPoint>,
}

static NEXT_CONFIG_ID: AtomicU32 = AtomicU32::new(1);

impl Default for RequestConfiguration {
    fn default() -> Self {
        Self {
            buffers: BTreeMap::new(),
            timeout_ms: 1000,
            config_id: NEXT_CONFIG_ID.fetch_add(1, Ordering::Relaxed),
            instrumentation_points: Vec::new(),
        }
    }
}

impl RequestConfiguration {
    pub fn new() -> Self { Self::default() }

    pub fn set_buffer(&mut self, operand_index: u32, addr: BaseAddress) {
        self.buffers.insert(operand_index, addr);
    }

    pub fn get_buffer(&self, operand_index: u32) -> Option<BaseAddress> {
        self.buffers.get(&operand_index).cloned()
    }

    pub fn set_instrumentation_points(&mut self, pts: &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint]) {
        self.instrumentation_points = pts.to_vec();
    }

    pub fn get_instrumentation_points(&self) -> &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint] {
        &self.instrumentation_points
    }
}
