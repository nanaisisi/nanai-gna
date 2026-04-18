/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `Layer` and related types.
use crate::common::BaseAddress;
use crate::gna_lib::buffer_map::BufferMap;

#[derive(Debug, Clone)]
pub struct Layer {
    pub buffers: BufferMap,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            buffers: BufferMap::new(),
        }
    }

    pub fn with_buffers(buffers: BufferMap) -> Self {
        Self { buffers }
    }

    pub fn set_buffer(&mut self, operand_index: u32, address: BaseAddress) {
        self.buffers.insert(operand_index, address);
    }

    pub fn get_buffer(&self, operand_index: u32) -> Option<BaseAddress> {
        self.buffers.get(operand_index)
    }

    pub fn has_buffer(&self, operand_index: u32) -> bool {
        self.buffers.get(operand_index).is_some()
    }

    pub fn clear_buffers(&mut self) {
        self.buffers = BufferMap::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BaseAddress;

    #[test]
    fn layer_can_store_and_retrieve_buffers() {
        let mut layer = Layer::new();
        let addr = BaseAddress::from_ptr(0x1000usize as *mut u8);

        layer.set_buffer(0, addr);
        assert_eq!(layer.get_buffer(0), Some(addr));
        assert!(layer.has_buffer(0));
        assert!(!layer.has_buffer(1));
    }

    #[test]
    fn layer_can_clear_buffers() {
        let mut layer = Layer::new();
        let addr = BaseAddress::from_ptr(0x2000usize as *mut u8);
        layer.set_buffer(1, addr);

        layer.clear_buffers();
        assert!(!layer.has_buffer(1));
    }
}
