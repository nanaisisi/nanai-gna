/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::driver_interface::{DriverPerf, DriverSubmissionResult, HardwarePerf};
use crate::gna_lib::hardware_request::HardwareRequest;
use std::mem::size_of;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GnaComputeCfg {
    hw_perf_encoding: u8,
    reserved: [u8; 7],
    buffers_ptr: u64,
    buffer_count: u32,
    reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GnaBuffer {
    handle: u32,
    offset: u32,
    size: u32,
    patches_ptr: u64,
    patch_count: u32,
    reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GnaMemoryPatch {
    offset: u32,
    size: u32,
    value: u32,
    reserved: u32,
}

#[derive(Debug)]
pub struct LinuxDriverInterface {
    device_index: u32,
    opened: bool,
}

impl LinuxDriverInterface {
    pub fn new(device_index: u32) -> Self {
        Self {
            device_index,
            opened: false,
        }
    }

    pub fn open(&mut self) -> bool {
        self.opened = true;
        true
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }

    pub fn create_request_descriptor(&self, hardware_request: &mut HardwareRequest) {
        let mut total_size = size_of::<GnaComputeCfg>();
        total_size += hardware_request.driver_memory_objects.len() * size_of::<GnaBuffer>();
        total_size += hardware_request
            .driver_memory_objects
            .iter()
            .map(|buffer| buffer.patches.len() * size_of::<GnaMemoryPatch>())
            .sum::<usize>();

        total_size = round_up(total_size, std::mem::align_of::<u64>());
        let mut calculation_data = vec![0u8; total_size];
        let base_ptr = calculation_data.as_mut_ptr();

        unsafe {
            let cfg_ptr = base_ptr as *mut GnaComputeCfg;
            ptr::write_unaligned(
                cfg_ptr,
                GnaComputeCfg {
                    hw_perf_encoding: hardware_request.hw_perf_encoding,
                    buffers_ptr: size_of::<GnaComputeCfg>() as u64,
                    buffer_count: hardware_request.driver_memory_objects.len() as u32,
                    ..Default::default()
                },
            );

            let mut buffer_offset = size_of::<GnaComputeCfg>();
            let mut patch_offset = buffer_offset
                + hardware_request.driver_memory_objects.len() * size_of::<GnaBuffer>();

            for buffer in &hardware_request.driver_memory_objects {
                let buffer_ptr = base_ptr.add(buffer_offset) as *mut GnaBuffer;
                ptr::write_unaligned(
                    buffer_ptr,
                    GnaBuffer {
                        handle: buffer.id,
                        offset: 0,
                        size: buffer.size,
                        patches_ptr: patch_offset as u64,
                        patch_count: buffer.patches.len() as u32,
                        ..Default::default()
                    },
                );

                buffer_offset += size_of::<GnaBuffer>();

                for patch in &buffer.patches {
                    let patch_ptr = base_ptr.add(patch_offset) as *mut GnaMemoryPatch;
                    ptr::write_unaligned(
                        patch_ptr,
                        GnaMemoryPatch {
                            offset: patch.offset,
                            size: patch.size as u32,
                            value: patch.value,
                            ..Default::default()
                        },
                    );
                    patch_offset += size_of::<GnaMemoryPatch>();
                }
            }
        }

        hardware_request.calculation_data = calculation_data;
        hardware_request.submit_ready = true;
    }

    pub fn submit_request(&self, hardware_request: &mut HardwareRequest) -> DriverSubmissionResult {
        self.create_request_descriptor(hardware_request);

        DriverSubmissionResult {
            status: if hardware_request.submit_ready { 0 } else { 1 },
            driver_perf: DriverPerf {
                preprocessing: 0,
                processing: 0,
                device_request_completed: 0,
                completion: 0,
            },
            hardware_perf: HardwarePerf { total: 0, stall: 0 },
        }
    }
}

fn round_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_lib::hardware_request::{HardwareRequest, MemoryPatch};
    use std::mem::size_of;

    #[test]
    fn linux_driver_interface_create_request_descriptor_sets_submit_ready() {
        let mut request = HardwareRequest::new(
            crate::gna_lib::request_configuration::RequestConfiguration::new(),
        );
        request.set_driver_buffer(1, 128);
        request.add_patch(MemoryPatch {
            offset: 16,
            value: 0x1000,
            size: 4,
        });

        let driver = LinuxDriverInterface::new(0);
        driver.create_request_descriptor(&mut request);

        assert!(request.submit_ready);
        assert_eq!(
            request.calculation_size(),
            round_up(
                size_of::<GnaComputeCfg>() + size_of::<GnaBuffer>() + size_of::<GnaMemoryPatch>(),
                std::mem::align_of::<u64>()
            )
        );
    }

    #[test]
    fn linux_driver_interface_submit_request_returns_success() {
        let mut request = HardwareRequest::new(
            crate::gna_lib::request_configuration::RequestConfiguration::new(),
        );
        request.set_driver_buffer(1, 128);

        let driver = LinuxDriverInterface::new(0);
        let result = driver.submit_request(&mut request);

        assert_eq!(result.status, 0);
        assert!(request.submit_ready);
    }
}
