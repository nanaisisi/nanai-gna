/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Memory management skeleton (Memory / MemoryContainer)
use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::gna_lib::driver_interface::DriverInterface;

#[derive(Debug, Default)]
pub struct Memory {
    mapped: bool,
}

impl Memory {
    pub fn alloc(&mut self, _bytes: usize) -> BaseAddress {
        BaseAddress::null()
    }

    pub fn map(&mut self, driver_interface: &DriverInterface) -> bool {
        if driver_interface.is_open() {
            self.mapped = true;
            true
        } else {
            false
        }
    }

    pub fn unmap(&mut self, driver_interface: &DriverInterface) -> bool {
        if self.mapped && driver_interface.is_open() {
            self.mapped = false;
            true
        } else {
            false
        }
    }

    pub fn is_mapped(&self) -> bool {
        self.mapped
    }
}

#[derive(Debug)]
pub struct MemoryContainer;
