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

use crate::gna_rs::common::gna_drv_api::GnaDriver;
use crate::gna_rs::common::gna_exception::Result as GnaResult;
use crate::gna_rs::common::{BaseAddress, SoftwareDriver};

/// Allocate memory via the default driver (software stub for now).
pub fn gna2_memory_alloc(bytes_requested: usize) -> GnaResult<BaseAddress> {
    // Prefer Linux driver when available (runtime detection), otherwise fall back to software driver
    #[cfg(unix)]
    {
        let linux = crate::gna_rs::common::gna_drv_api::LinuxGnaDriver::new();
        if linux.get_device_count().unwrap_or(0) > 0 {
            return linux.memory_alloc(bytes_requested);
        }
    }

    let driver = SoftwareDriver::default();
    GnaDriver::memory_alloc(&driver, bytes_requested)
}

/// Free memory via driver
pub fn gna2_memory_free(addr: BaseAddress) -> GnaResult<()> {
    #[cfg(unix)]
    {
        let linux = crate::gna_rs::common::gna_drv_api::LinuxGnaDriver::new();
        if linux.get_device_count().unwrap_or(0) > 0 {
            return linux.memory_free(addr);
        }
    }
    let driver = SoftwareDriver::default();
    GnaDriver::memory_free(&driver, addr)
}
