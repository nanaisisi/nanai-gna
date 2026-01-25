use crate::gna_lib::BufferMap;
use crate::common::BaseAddress;

/// Wrapper implementing higher-level behaviors mirroring original `BufferMap` usage.
#[derive(Debug, Default)]
pub struct BufferMapImpl {
    inner: BufferMap,
}

impl BufferMapImpl {
    pub fn new() -> Self { Self { inner: BufferMap::new() } }

    pub fn map_operand(&mut self, operand: u32, addr: BaseAddress) {
        self.inner.insert(operand, addr);
    }

    pub fn get_address(&self, operand: u32) -> Option<BaseAddress> { self.inner.get(operand) }
}
