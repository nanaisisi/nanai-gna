//! Memory management skeleton (Memory / MemoryContainer)

use crate::common::BaseAddress;

#[derive(Debug)]
pub struct Memory {
    // tracking allocations
}

impl Memory {
    pub fn alloc(&self, _bytes: usize) -> BaseAddress { BaseAddress::null() }
}

#[derive(Debug)]
pub struct MemoryContainer;
