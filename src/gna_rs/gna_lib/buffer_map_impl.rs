use crate::gna_rs::common::BaseAddress;
/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_lib::BufferMap;

/// Wrapper implementing higher-level behaviors mirroring original `BufferMap` usage.
#[derive(Debug, Default)]
pub struct BufferMapImpl {
    inner: BufferMap,
}

impl BufferMapImpl {
    pub fn new() -> Self {
        Self {
            inner: BufferMap::new(),
        }
    }

    pub fn map_operand(&mut self, operand: u32, addr: BaseAddress) {
        self.inner.insert(operand, addr);
    }

    pub fn get_address(&self, operand: u32) -> Option<BaseAddress> {
        self.inner.get(operand)
    }
}
