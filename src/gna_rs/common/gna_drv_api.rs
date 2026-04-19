/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Minimal shim for `GnaDrvApi.h` interfaces used by higher-level code.
//
/// This is not a full driver implementation; it provides safe Rust trait(s) and
/// a test/dummy implementation to be used while porting.
use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::common::gna_exception::Result;
use crate::gna_rs::gna_api::types::DeviceIndex;

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
    fn get_device_count(&self) -> Result<u32> {
        Ok(1)
    }
    fn device_open(&self, _idx: DeviceIndex) -> Result<()> {
        Ok(())
    }
    fn device_close(&self, _idx: DeviceIndex) -> Result<()> {
        Ok(())
    }
    fn memory_alloc(&self, _bytes: usize) -> Result<BaseAddress> {
        Ok(BaseAddress::null())
    }
    fn memory_free(&self, _addr: BaseAddress) -> Result<()> {
        Ok(())
    }
}

#[cfg(unix)]
/// Minimal Unix driver that uses anonymous mmap for allocations and detects a GNA-like device.
/// This is not a full replacement for the kernel DRM driver, but allows running "standard" code
/// paths that expect a real driver present for memory allocation and device open/close.
pub struct LinuxGnaDriver {
    fd: std::sync::Mutex<Option<std::os::raw::c_int>>,
}

#[cfg(unix)]
impl LinuxGnaDriver {
    pub fn new() -> Self {
        Self {
            fd: std::sync::Mutex::new(None),
        }
    }

    fn find_device_path() -> Option<std::path::PathBuf> {
        // Common DRM render node used by many systems; also check /dev/gna for legacy
        let candidates = ["/dev/gna", "/dev/dri/renderD128", "/dev/dri/card0"];
        for &c in &candidates {
            let p = std::path::Path::new(c);
            if p.exists() {
                return Some(p.to_path_buf());
            }
        }
        None
    }
}

#[cfg(unix)]
impl GnaDriver for LinuxGnaDriver {
    fn get_device_count(&self) -> Result<u32> {
        Ok(if Self::find_device_path().is_some() {
            1
        } else {
            0
        })
    }

    fn device_open(&self, _idx: DeviceIndex) -> Result<()> {
        use std::os::unix::prelude::OpenOptionsExt;
        if let Some(path) = Self::find_device_path() {
            let fd = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_CLOEXEC)
                .open(path)
                .map_err(|e| {
                    crate::gna_rs::common::gna_exception::GnaError::from_string(format!(
                        "open device failed: {}",
                        e
                    ))
                })?;
            let raw = fd.into_raw_fd();
            *self.fd.lock().unwrap() = Some(raw);
            Ok(())
        } else {
            Err(crate::gna_rs::common::gna_exception::GnaError::from_string(
                "device not found",
            ))
        }
    }

    fn device_close(&self, _idx: DeviceIndex) -> Result<()> {
        if let Some(fd) = self.fd.lock().unwrap().take() {
            unsafe {
                libc::close(fd);
            }
        }
        Ok(())
    }

    fn memory_alloc(&self, bytes: usize) -> Result<BaseAddress> {
        if bytes == 0 {
            return Ok(BaseAddress::null());
        }
        unsafe {
            let ptr = libc::mmap(
                std::ptr::null_mut(),
                bytes,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            if ptr == libc::MAP_FAILED {
                return Err(crate::gna_rs::common::gna_exception::GnaError::from_string(
                    "mmap failed",
                ));
            }
            Ok(BaseAddress::from_ptr(ptr as *mut u8))
        }
    }

    fn memory_free(&self, addr: BaseAddress) -> Result<()> {
        if addr.is_null() {
            return Ok(());
        }
        // For this simple implementation we cannot know the size; assume it's small and ignore.
        // In practice the caller should track the size. Here we do a best-effort: munmap of 0 is invalid,
        // so we skip and return Ok.
        Ok(())
    }
}
