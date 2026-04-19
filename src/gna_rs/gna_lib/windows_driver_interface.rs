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
struct GnaInferenceConfigIn {
    hw_perf_encoding: u8,
    reserved: [u8; 7],
    buffer_count: u32,
    reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GnaMemoryBuffer {
    memory_id: u32,
    offset: u32,
    size: u32,
    patch_count: u32,
    reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GnaMemoryPatch {
    offset: u32,
    size: u32,
    reserved: u32,
    reserved2: u32,
}

#[derive(Debug)]
pub struct WindowsDriverInterface {
    device_index: u32,
    opened: bool,
}

impl WindowsDriverInterface {
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
        let buffer_count = hardware_request.driver_memory_objects.len();
        let mut total_size =
            size_of::<GnaInferenceConfigIn>() + buffer_count * size_of::<GnaMemoryBuffer>();

        for buffer in &hardware_request.driver_memory_objects {
            total_size += buffer.patches.len() * size_of::<GnaMemoryPatch>();
            for patch in &buffer.patches {
                total_size += patch.size;
            }
        }

        total_size = total_size.max(size_of::<GnaInferenceConfigIn>());
        total_size = round_up(total_size, std::mem::align_of::<u64>());

        let mut calculation_data = vec![0u8; total_size];
        let base_ptr = calculation_data.as_mut_ptr();

        unsafe {
            let input_ptr = base_ptr as *mut GnaInferenceConfigIn;
            ptr::write_unaligned(
                input_ptr,
                GnaInferenceConfigIn {
                    hw_perf_encoding: hardware_request.hw_perf_encoding,
                    buffer_count: buffer_count as u32,
                    ..Default::default()
                },
            );

            let mut buffer_offset = size_of::<GnaInferenceConfigIn>();
            let mut patch_offset = buffer_offset + buffer_count * size_of::<GnaMemoryBuffer>();

            for buffer in &hardware_request.driver_memory_objects {
                let buffer_ptr = base_ptr.add(buffer_offset) as *mut GnaMemoryBuffer;
                ptr::write_unaligned(
                    buffer_ptr,
                    GnaMemoryBuffer {
                        memory_id: buffer.id,
                        offset: 0,
                        size: buffer.size,
                        patch_count: buffer.patches.len() as u32,
                        ..Default::default()
                    },
                );
                buffer_offset += size_of::<GnaMemoryBuffer>();

                for patch in &buffer.patches {
                    let patch_ptr = base_ptr.add(patch_offset) as *mut GnaMemoryPatch;
                    ptr::write_unaligned(
                        patch_ptr,
                        GnaMemoryPatch {
                            offset: patch.offset,
                            size: patch.size as u32,
                            ..Default::default()
                        },
                    );

                    let data_ptr = base_ptr.add(patch_offset + size_of::<GnaMemoryPatch>());
                    let value_bytes = patch.value.to_ne_bytes();
                    let copy_size = std::cmp::min(patch.size, value_bytes.len());
                    ptr::copy_nonoverlapping(value_bytes.as_ptr(), data_ptr, copy_size);

                    patch_offset += size_of::<GnaMemoryPatch>() + patch.size;
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
    fn windows_driver_interface_open_marks_instance_as_open() {
        let mut driver = WindowsDriverInterface::new(0);
        assert!(!driver.is_open());

        assert!(driver.open());
        assert!(driver.is_open());
    }

    #[test]
    fn windows_driver_interface_create_request_descriptor_sets_submit_ready() {
        let mut request = HardwareRequest::new(
            crate::gna_lib::request_configuration::RequestConfiguration::new(),
        );
        request.set_driver_buffer(2, 256);
        request.add_patch(MemoryPatch {
            offset: 24,
            value: 0xdeadbeef,
            size: 4,
        });

        let driver = WindowsDriverInterface::new(0);
        driver.create_request_descriptor(&mut request);

        assert!(request.submit_ready);
        assert_eq!(
            request.calculation_data().len(),
            round_up(
                size_of::<GnaInferenceConfigIn>()
                    + size_of::<GnaMemoryBuffer>()
                    + size_of::<GnaMemoryPatch>()
                    + 4,
                std::mem::align_of::<u64>()
            )
        );
    }

    #[test]
    fn windows_driver_interface_submit_request_returns_success() {
        let mut request = HardwareRequest::new(
            crate::gna_lib::request_configuration::RequestConfiguration::new(),
        );
        request.set_driver_buffer(3, 512);

        let driver = WindowsDriverInterface::new(0);
        let result = driver.submit_request(&mut request);

        assert_eq!(result.status, 0);
        assert!(request.submit_ready);
    }
}
