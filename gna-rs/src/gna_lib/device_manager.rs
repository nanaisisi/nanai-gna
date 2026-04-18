/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::collections::BTreeMap;

use crate::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_lib::device::Device;
use crate::gna_lib::export_device::ExportDevice;
use crate::gna_lib::hybrid_device::HybridDevice;
use crate::gna_lib::memory::Memory;

/// Simplified Rust port of the GNA `DeviceManager` helper.
#[derive(Debug)]
pub struct DeviceManager {
    capabilities: BTreeMap<u32, Gna2DeviceVersion>,
    devices: BTreeMap<u32, DeviceContext>,
    memory_objects: Vec<Memory>,
    export_devices_count: u32,
}

#[derive(Debug)]
enum DeviceHandle {
    Device(Device),
    Export(ExportDevice),
    Hybrid(HybridDevice),
}

impl DeviceHandle {
    fn as_device(&self) -> &Device {
        match self {
            DeviceHandle::Device(device) => device,
            DeviceHandle::Export(export_device) => &export_device.device,
            DeviceHandle::Hybrid(hybrid_device) => hybrid_device.get_device(),
        }
    }

    fn as_mut_device(&mut self) -> &mut Device {
        match self {
            DeviceHandle::Device(device) => device,
            DeviceHandle::Export(export_device) => &mut export_device.device,
            DeviceHandle::Hybrid(hybrid_device) => hybrid_device.get_device_mut(),
        }
    }

    fn as_export(&self) -> Option<&ExportDevice> {
        match self {
            DeviceHandle::Export(export_device) => Some(export_device),
            _ => None,
        }
    }

    fn is_hybrid(&self) -> bool {
        matches!(self, DeviceHandle::Hybrid(_))
    }
}

#[derive(Debug)]
struct DeviceContext {
    handle: DeviceHandle,
    reference_count: u32,
}

impl DeviceContext {
    fn new(handle: DeviceHandle) -> Self {
        Self {
            handle,
            reference_count: 0,
        }
    }

    fn increment(&mut self) -> u32 {
        self.reference_count += 1;
        self.reference_count
    }

    fn decrement(&mut self) -> u32 {
        assert!(self.reference_count > 0, "Device reference count underflow");
        self.reference_count -= 1;
        self.reference_count
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        let mut capabilities = BTreeMap::new();
        capabilities.insert(0, Gna2DeviceVersion(0x30));

        Self {
            capabilities,
            devices: BTreeMap::new(),
            memory_objects: Vec::new(),
            export_devices_count: 0,
        }
    }
}

impl DeviceManager {
    /// Enumerate available devices in the current system stub.
    pub fn enumerate() -> Vec<Device> {
        vec![Device::new(Gna2DeviceVersion(0x30))]
    }

    /// Return the number of enumerated devices.
    pub fn device_count() -> usize {
        1
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_device_count(&self) -> u32 {
        self.capabilities.len() as u32
    }

    pub fn get_device_version(&self, device_index: u32) -> Gna2DeviceVersion {
        if let Some(device_context) = self.devices.get(&device_index) {
            return device_context.handle.as_device().get_version();
        }
        *self
            .capabilities
            .get(&device_index)
            .expect("invalid device index")
    }

    pub fn open_device(&mut self, device_index: u32) {
        assert!(
            device_index < self.get_device_count(),
            "invalid device index"
        );

        if !self.is_open(device_index) {
            let mut device = HybridDevice::create(device_index);
            self.map_all_to_device(&mut device);
            self.devices.insert(
                device_index,
                DeviceContext::new(DeviceHandle::Hybrid(device)),
            );
        }

        let device_context = self
            .devices
            .get_mut(&device_index)
            .expect("device should exist");
        device_context.increment();
    }

    pub fn close_device(&mut self, device_index: u32) {
        if device_index >= self.get_device_count() {
            let removed = self.devices.remove(&device_index).is_some();
            assert!(removed, "invalid device index");
            self.export_devices_count = self.export_devices_count.saturating_sub(1);
            return;
        }

        let remaining = {
            let device_context = self
                .devices
                .get_mut(&device_index)
                .expect("device must be opened");
            device_context.decrement()
        };

        if remaining == 0 {
            let mut device_context = self
                .devices
                .remove(&device_index)
                .expect("device must be opened");
            self.unmap_all_memory_objects_from_handle(&mut device_context.handle);
        }
    }

    pub fn create_export_device(&mut self, target_device_version: Gna2DeviceVersion) -> u32 {
        assert!(
            target_device_version != Gna2DeviceVersion(0),
            "invalid target device version"
        );

        let index = self.get_device_count() + self.export_devices_count;
        let device = ExportDevice::new(target_device_version);
        let emplaced = self
            .devices
            .insert(index, DeviceContext::new(DeviceHandle::Export(device)));
        assert!(emplaced.is_none(), "export device index already exists");
        self.export_devices_count += 1;
        index
    }

    pub fn get_device_for_export(&self, device_index: u32) -> &ExportDevice {
        self.devices
            .get(&device_index)
            .and_then(|context| context.handle.as_export())
            .expect("not an export device")
    }

    pub fn get_device_for_export_mut(&mut self, device_index: u32) -> &mut ExportDevice {
        self.devices
            .get_mut(&device_index)
            .and_then(|context| match &mut context.handle {
                DeviceHandle::Export(export_device) => Some(export_device),
                _ => None,
            })
            .expect("not an export device")
    }

    pub fn is_open(&self, device_index: u32) -> bool {
        self.devices.contains_key(&device_index)
    }

    pub fn get_device_for_model(&self, model_id: u32) -> &Device {
        self.try_get_device_for_model(model_id)
            .expect("device for model not found")
    }

    pub fn try_get_device_for_model(&self, model_id: u32) -> Option<&Device> {
        self.devices.values().find_map(|device_context| {
            let device = device_context.handle.as_device();
            if device.has_model(model_id) {
                Some(device)
            } else {
                None
            }
        })
    }

    pub fn allocate_memory(&mut self, _device_index: u32, _requested_size: u32) -> &mut Memory {
        self.create_internal_memory()
    }

    pub fn create_internal_memory(&mut self) -> &mut Memory {
        self.memory_objects.push(Memory::default());
        self.memory_objects
            .last_mut()
            .expect("memory object just added")
    }

    pub fn find_memory(&self, _buffer: *const u8) -> Option<usize> {
        None
    }

    pub fn free_memory(&mut self, _buffer: *const u8) {
        // no-op in this simplified implementation
    }

    pub fn map_memory_to_all(&mut self, memory_object: &mut Memory) {
        for device_context in self.devices.values_mut() {
            if let DeviceHandle::Hybrid(hybrid_device) = &mut device_context.handle {
                hybrid_device.map_memory(memory_object);
            }
        }
    }

    pub fn unmap_memory_from_all_devices(&mut self, memory_object: &mut Memory) {
        for device_context in self.devices.values_mut() {
            if let DeviceHandle::Hybrid(hybrid_device) = &mut device_context.handle {
                hybrid_device.unmap_memory(memory_object);
            }
        }
    }

    pub fn get_device_for_request_config_id(&self, request_config_id: u32) -> &Device {
        self.try_get_device_for_request_config_id(request_config_id)
            .expect("device for request config id not found")
    }

    pub fn try_get_device_for_request_config_id(&self, request_config_id: u32) -> Option<&Device> {
        self.devices.values().find_map(|device_context| {
            let device = device_context.handle.as_device();
            if device.has_request_config_id(request_config_id) {
                Some(device)
            } else {
                None
            }
        })
    }

    pub fn get_device_for_request_id(&self, request_id: u32) -> &Device {
        self.devices
            .values()
            .find_map(|device_context| {
                let device = device_context.handle.as_device();
                if device.has_request_id(request_id) {
                    Some(device)
                } else {
                    None
                }
            })
            .expect("device for request id not found")
    }

    fn map_all_to_device(&mut self, device: &mut HybridDevice) {
        for memory_object in self.memory_objects.iter_mut() {
            device.map_memory(memory_object);
        }
    }

    fn unmap_all_memory_objects_from_handle(&mut self, handle: &mut DeviceHandle) {
        if let DeviceHandle::Hybrid(hybrid_device) = handle {
            for memory_object in self.memory_objects.iter_mut() {
                hybrid_device.unmap_memory(memory_object);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_api::device_api::Gna2DeviceVersion;
    use crate::gna_api::model_api::Gna2Model;

    #[test]
    fn device_manager_enumerate_returns_at_least_one_device() {
        let devices = DeviceManager::enumerate();
        assert_eq!(devices.len(), 1);
    }

    #[test]
    fn device_manager_device_count_returns_one() {
        assert_eq!(DeviceManager::device_count(), 1);
    }

    #[test]
    fn device_manager_open_and_close_device() {
        let mut manager = DeviceManager::new();
        manager.open_device(0);
        assert!(manager.is_open(0));
        manager.close_device(0);
        assert!(!manager.is_open(0));
    }

    #[test]
    fn device_manager_create_export_device_returns_export_index() {
        let mut manager = DeviceManager::new();
        let export_index = manager.create_export_device(Gna2DeviceVersion(0x30));
        assert_eq!(export_index, manager.get_device_count());
        let export_device = manager.get_device_for_export_mut(export_index);
        let model = Gna2Model::new();
        export_device.load_model(&model);
        assert!(export_device.export());
    }
}
