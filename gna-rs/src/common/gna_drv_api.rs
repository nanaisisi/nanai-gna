/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Minimal shim for `GnaDrvApi.h` interfaces used by higher-level code.
//
/// This is not a full driver implementation; it provides safe Rust trait(s) and
/// a test/dummy implementation to be used while porting.

use crate::common::BaseAddress;
use crate::gna_api::types::DeviceIndex;
use crate::common::gna_exception::Result;

pub trait GnaDriver {
    fn get_device_count(&self) -> Result<u32>;
    fn device_open(&self, idx: DeviceIndex) -> Result<()>;
    fn device_close(&self, idx: DeviceIndex) -> Result<()>;
    fn memory_alloc(&self, bytes: usize) -> Result<BaseAddress>;
    fn memory_free(&self, addr: BaseAddress) -> Result<()>;
}

/// Dummy software driver used for tests / Rust backend.
#[derive(Default)]
pub struct SoftwareDriver;

impl GnaDriver for SoftwareDriver {
    fn get_device_count(&self) -> Result<u32> { Ok(1) }
    fn device_open(&self, _idx: DeviceIndex) -> Result<()> { Ok(()) }
    fn device_close(&self, _idx: DeviceIndex) -> Result<()> { Ok(()) }
    fn memory_alloc(&self, _bytes: usize) -> Result<BaseAddress> { Ok(BaseAddress::null()) }
    fn memory_free(&self, _addr: BaseAddress) -> Result<()> { Ok(()) }
}
