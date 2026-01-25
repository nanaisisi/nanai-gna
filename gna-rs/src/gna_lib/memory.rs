/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Memory management skeleton (Memory / MemoryContainer)

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
