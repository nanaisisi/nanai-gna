/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::model_api::Gna2Model;
use crate::gna_rs::gna_lib::acceleration_detector::AccelerationDetector;
use crate::gna_rs::gna_lib::device::Device;
use crate::gna_rs::gna_lib::driver_interface::DriverInterface;
use crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_rs::gna_lib::hybrid_model::HybridModel;
use crate::gna_rs::gna_lib::memory::Memory;

/// Simplified Rust port of the GNA `HybridDevice` helper.
#[derive(Debug)]
pub struct HybridDevice {
    device: Device,
    driver_interface: DriverInterface,
    hw_capabilities: HardwareCapabilities,
    detector: AccelerationDetector,
}

impl HybridDevice {
    pub fn create(index: u32) -> Self {
        let mut driver_interface = DriverInterface::new(index);
        driver_interface.open();
        let hw_capabilities = HardwareCapabilities;
        let device = Device::new(driver_interface.device_version());

        Self {
            device,
            driver_interface,
            hw_capabilities,
            detector: AccelerationDetector,
        }
    }

    pub fn map_memory(&mut self, memory: &mut Memory) {
        if self.hw_capabilities.is_hardware_supported() {
            memory.map(&self.driver_interface);
        }
    }

    pub fn unmap_memory(&mut self, memory: &mut Memory) -> bool {
        if self.hw_capabilities.is_hardware_supported() {
            memory.unmap(&self.driver_interface)
        } else {
            false
        }
    }

    pub fn load_model(&mut self, model: &Gna2Model) -> u32 {
        let hybrid_model = HybridModel::new(
            model.clone(),
            self.detector.clone(),
            self.hw_capabilities,
            self.driver_interface.clone(),
        );

        let compiled = hybrid_model.compiled_model().clone();
        self.device.load_compiled_model(compiled)
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_device_mut(&mut self) -> &mut Device {
        &mut self.device
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
    use crate::gna_rs::gna_api::model_api::Gna2Model;

    #[test]
    fn hybrid_device_create_opens_driver_interface() {
        let device = HybridDevice::create(0);
        assert_eq!(device.get_device().get_version(), Gna2DeviceVersion(0x30));
    }

    #[test]
    fn hybrid_device_map_and_unmap_memory_when_supported() {
        let mut device = HybridDevice::create(0);
        let mut memory = Memory::default();

        device.map_memory(&mut memory);
        assert!(memory.is_mapped());

        assert!(device.unmap_memory(&mut memory));
        assert!(!memory.is_mapped());
    }

    #[test]
    fn hybrid_device_loads_model_and_stores_compiled_model() {
        let mut device = HybridDevice::create(0);
        let model = Gna2Model::new();

        let model_id = device.load_model(&model);
        assert!(device.get_device().has_model(model_id));
    }
}
