/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Skeleton for `gna2-memory-api.h`.

/// Abstract representation of allocated memory returned by the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Gna2MemoryHandle(pub usize);

/// Simple placeholder for memory allocation flags / helpers
pub const GNA2_MEMORY_DEFAULT: u32 = 0;

use crate::common::{SoftwareDriver, BaseAddress};
use crate::common::gna_exception::Result as GnaResult;
use crate::common::gna_drv_api::GnaDriver;

/// Allocate memory via the default driver (software stub for now).
pub fn Gna2MemoryAlloc(bytes_requested: usize) -> GnaResult<BaseAddress> {
    let driver = SoftwareDriver::default();
    GnaDriver::memory_alloc(&driver, bytes_requested)
}

/// Free memory via driver
pub fn Gna2MemoryFree(addr: BaseAddress) -> GnaResult<()> {
    let driver = SoftwareDriver::default();
    GnaDriver::memory_free(&driver, addr)
}
