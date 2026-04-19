/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::collections::BTreeMap;

use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_rs::gna_api::model_api::Gna2Model;
use crate::gna_rs::gna_lib::acceleration_detector::AccelerationDetector;
use crate::gna_rs::gna_lib::active_list::ActiveList;
use crate::gna_rs::gna_lib::compiled_model::CompiledModel;
use crate::gna_rs::gna_lib::driver_interface::DriverInterface;
use crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_rs::gna_lib::request::{enqueue_request, get_request_state, wait_request};
use crate::gna_rs::gna_lib::request_builder::RequestBuilder;
use crate::gna_rs::gna_lib::request_configuration::RequestConfiguration;
use crate::gna_rs::gna_lib::request_handler::RequestHandler;

/// Simplified Rust port of the GNA `Device` helper.
#[derive(Debug)]
pub struct Device {
    version: Gna2DeviceVersion,
    number_of_threads: u32,
    driver_interface: DriverInterface,
    hardware_capabilities: Option<HardwareCapabilities>,
    acceleration_detector: AccelerationDetector,
    request_builder: RequestBuilder,
    request_handler: RequestHandler,
    models: BTreeMap<u32, CompiledModel>,
}

impl Device {
    pub fn new(version: Gna2DeviceVersion) -> Self {
        Self {
            version,
            number_of_threads: 1,
            driver_interface: DriverInterface::new(0),
            hardware_capabilities: Some(HardwareCapabilities),
            acceleration_detector: AccelerationDetector,
            request_builder: RequestBuilder::new(),
            request_handler: RequestHandler::new(),
            models: BTreeMap::new(),
        }
    }

    pub fn get_version(&self) -> Gna2DeviceVersion {
        self.version
    }

    pub fn get_number_of_threads(&self) -> u32 {
        self.number_of_threads
    }

    pub fn set_number_of_threads(&mut self, thread_count: u32) {
        self.number_of_threads = thread_count;
    }

    pub fn load_model(&mut self, model: &Gna2Model) -> u32 {
        let compiled = CompiledModel::new(model.clone());
        let model_id = compiled.id();
        self.models.insert(model_id, compiled);
        model_id
    }

    pub fn load_compiled_model(&mut self, compiled_model: CompiledModel) -> u32 {
        let model_id = compiled_model.id();
        self.models.insert(model_id, compiled_model);
        model_id
    }

    pub fn get_model(&self, model_id: u32) -> Option<&CompiledModel> {
        self.models.get(&model_id)
    }

    pub fn release_model(&mut self, model_id: u32) -> bool {
        self.models.remove(&model_id).is_some()
    }

    pub fn attach_buffer(
        &mut self,
        config_id: u32,
        operand_index: u32,
        layer_index: u32,
        address: BaseAddress,
    ) -> bool {
        self.request_builder
            .attach_buffer(config_id, layer_index, operand_index, address)
    }

    pub fn create_configuration(&mut self, model_id: u32) -> Option<u32> {
        if self.models.contains_key(&model_id) {
            let config = RequestConfiguration::new();
            let config_id = config.config_id;
            self.request_builder.create_configuration(config);
            Some(config_id)
        } else {
            None
        }
    }

    pub fn release_configuration(&mut self, config_id: u32) -> bool {
        self.request_builder.release_configuration(config_id)
    }

    pub fn is_version_consistent(&self, device_version: Gna2DeviceVersion) -> bool {
        self.version == device_version
    }

    pub fn enforce_acceleration(
        &mut self,
        config_id: u32,
        acceleration_mode: crate::gna_rs::gna_api::inference_api::Gna2AccelerationMode,
    ) -> bool {
        if let Some(config) = self.request_builder.get_configuration_mut(config_id) {
            config.set_acceleration_mode(acceleration_mode);
            true
        } else {
            false
        }
    }

    pub fn attach_active_list(
        &mut self,
        config_id: u32,
        layer_index: u32,
        indices_count: u32,
        indices: &[u32],
    ) -> bool {
        if let Some(config) = self.request_builder.get_configuration_mut(config_id) {
            let mut active_list = ActiveList::new();
            for &index in indices.iter().take(indices_count as usize) {
                active_list.add(index as usize);
            }
            config.add_active_list(layer_index, active_list)
        } else {
            false
        }
    }

    pub fn propagate_request(&mut self, config_id: u32) -> Option<u32> {
        self.request_builder
            .create_request(config_id)
            .map(|request| enqueue_request(request.config))
    }

    pub fn wait_for_request(&self, request_id: u32, timeout_ms: u32) -> bool {
        wait_request(request_id, timeout_ms)
    }

    pub fn stop(&self) {
        // No op for the simplified stub.
    }

    pub fn assign_profiler_config_to_request_config(&mut self, _request_config_id: u32) -> bool {
        // Stub: profiler assignment is not supported in this simplified port.
        self.request_builder.has_configuration(_request_config_id)
    }

    pub fn has_model(&self, model_id: u32) -> bool {
        self.models.contains_key(&model_id)
    }

    pub fn has_request_config_id(&self, request_config_id: u32) -> bool {
        self.request_builder.has_configuration(request_config_id)
    }

    pub fn has_request_id(&self, request_id: u32) -> bool {
        get_request_state(request_id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use crate::gna_rs::common::BaseAddress;
    use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
    use crate::gna_rs::gna_api::inference_api::Gna2AccelerationMode;
    use crate::gna_rs::gna_api::model_api::Gna2Model;

    #[test]
    fn device_loads_and_releases_model() {
        let mut device = Device::new(Gna2DeviceVersion(0x30));
        let model = Gna2Model::new();
        let model_id = device.load_model(&model);

        assert!(device.has_model(model_id));
        assert!(device.release_model(model_id));
        assert!(!device.has_model(model_id));
    }

    #[test]
    fn device_creates_configuration_and_propagates_request() {
        let mut device = Device::new(Gna2DeviceVersion(0x30));
        let model = Gna2Model::new();
        let model_id = device.load_model(&model);
        let config_id = device
            .create_configuration(model_id)
            .expect("configuration should be created");

        assert!(device.has_request_config_id(config_id));
        assert!(device.enforce_acceleration(config_id, Gna2AccelerationMode::Software));

        let request_id = device
            .propagate_request(config_id)
            .expect("request should propagate");
        assert!(device.has_request_id(request_id));
        assert!(device.wait_for_request(request_id, 100));
    }

    #[test]
    fn device_version_consistency_checks_match() {
        let device = Device::new(Gna2DeviceVersion(0x30));
        assert!(device.is_version_consistent(Gna2DeviceVersion(0x30)));
        assert!(!device.is_version_consistent(Gna2DeviceVersion(0x20)));
    }

    #[test]
    fn device_attach_active_list_updates_request_config() {
        let mut device = Device::new(Gna2DeviceVersion(0x30));
        let model = Gna2Model::new();
        let model_id = device.load_model(&model);
        let config_id = device.create_configuration(model_id).unwrap();

        assert!(device.attach_active_list(config_id, 0, 2, &[10, 20]));
        let config = device.request_builder.get_configuration(config_id).unwrap();
        let layer_config = config.get_layer_configuration(0).unwrap();

        assert!(layer_config.has_active_list());
        assert_eq!(layer_config.get_active_list().unwrap().len(), 2);
        assert!(layer_config.get_active_list().unwrap().contains(10));
        assert!(layer_config.get_active_list().unwrap().contains(20));
    }
}
