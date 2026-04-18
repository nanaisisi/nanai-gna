/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Skeleton for `LayerConfiguration`.
use crate::common::BaseAddress;
use crate::gna_lib::buffer_map::BufferMap;

#[derive(Debug, Default, Clone)]
pub struct LayerConfiguration {
    pub buffers: BufferMap,
    // other config: activation lists, config lists
}

impl LayerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_buffer(&mut self, operand_index: u32, address: BaseAddress) {
        self.buffers.insert(operand_index, address);
    }

    pub fn emplace_buffer(&mut self, operand_index: u32, address: BaseAddress) -> bool {
        self.buffers.emplace(operand_index, address)
    }

    pub fn get_buffer(&self, operand_index: u32) -> Option<BaseAddress> {
        self.buffers.get(operand_index)
    }

    pub fn has_buffer(&self, operand_index: u32) -> bool {
        self.buffers.get(operand_index).is_some()
    }

    pub fn remove_buffer(&mut self, operand_index: u32) {
        self.buffers.erase(&operand_index);
    }

    pub fn clear(&mut self) {
        self.buffers = BufferMap::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BaseAddress;

    #[test]
    fn configuration_can_store_buffers() {
        let mut config = LayerConfiguration::new();
        let addr = BaseAddress::from_ptr(0x3000usize as *mut u8);
        config.set_buffer(2, addr);
        assert_eq!(config.get_buffer(2), Some(addr));
    }

    #[test]
    fn configuration_can_emplace_buffers_only_once() {
        let mut config = LayerConfiguration::new();
        let addr1 = BaseAddress::from_ptr(0x3000usize as *mut u8);
        let addr2 = BaseAddress::from_ptr(0x4000usize as *mut u8);

        assert!(config.emplace_buffer(4, addr1));
        assert!(!config.emplace_buffer(4, addr2));
        assert_eq!(config.get_buffer(4), Some(addr1));
    }

    #[test]
    fn configuration_can_remove_buffers() {
        let mut config = LayerConfiguration::new();
        let addr = BaseAddress::from_ptr(0x4000usize as *mut u8);
        config.set_buffer(3, addr);
        config.remove_buffer(3);
        assert!(!config.has_buffer(3));
    }

    #[test]
    fn configuration_can_be_cleared() {
        let mut config = LayerConfiguration::new();
        let addr = BaseAddress::from_ptr(0x4000usize as *mut u8);
        config.set_buffer(3, addr);
        config.clear();
        assert!(config.get_buffer(3).is_none());
    }
}
