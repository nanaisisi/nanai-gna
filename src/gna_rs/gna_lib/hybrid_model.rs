/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_rs::gna_api::inference_api::Gna2AccelerationMode;
use crate::gna_rs::gna_api::model_api::Gna2Model;
use crate::gna_rs::gna_lib::acceleration_detector::AccelerationDetector;
use crate::gna_rs::gna_lib::compiled_model::CompiledModel;
use crate::gna_rs::gna_lib::driver_interface::DriverInterface;
use crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_rs::gna_lib::hardware_model_scorable::HardwareModelScorable;
use crate::gna_rs::gna_lib::iscorable::IScorable;
use crate::gna_rs::gna_lib::memory::Memory;
use crate::gna_rs::gna_lib::memory_container::MemoryContainer;
use crate::gna_rs::gna_lib::request_configuration::RequestConfiguration;
use crate::gna_rs::gna_lib::software_model::SoftwareModel;
use crate::gna_rs::gna_lib::sub_model::{SubModel, SubModelType};
use std::collections::HashMap;

/// Simplified Rust port of the GNA `HybridModel` helper.
#[derive(Debug)]
pub struct HybridModel {
    compiled_model: CompiledModel,
    software_model: SoftwareModel,
    software_model_for_present_device: Option<SoftwareModel>,
    hardware_model: Option<HardwareModelScorable>,
    fully_hardware_compatible: bool,
    sub_models: HashMap<Gna2DeviceVersion, Vec<SubModel>>,
    hw_capabilities: HardwareCapabilities,
    _detector: AccelerationDetector,
}

impl HybridModel {
    pub fn new(
        model: Gna2Model,
        detector: AccelerationDetector,
        hw_capabilities: HardwareCapabilities,
        ddi: DriverInterface,
    ) -> Self {
        let compiled_model = CompiledModel::new(model.clone());
        let software_model = SoftwareModel::new(model.clone());

        let mut hybrid = Self {
            compiled_model,
            software_model,
            software_model_for_present_device: None,
            hardware_model: None,
            fully_hardware_compatible: false,
            sub_models: HashMap::new(),
            hw_capabilities,
            _detector: detector,
        };

        if hybrid.hw_capabilities.is_hardware_supported() {
            hybrid.build_hardware_model(ddi);
        }

        hybrid
    }

    fn build_hardware_model(&mut self, ddi: DriverInterface) {
        let device_version = self.hw_capabilities.get_device_version();
        let sub_models = self.get_sub_models(device_version);

        if sub_models.is_empty() {
            return;
        }

        self.software_model_for_present_device =
            Some(SoftwareModel::new(self.compiled_model.model.clone()));
        self.hardware_model = Some(HardwareModelScorable::new(
            &self.compiled_model,
            ddi,
            self.hw_capabilities,
            sub_models.clone(),
        ));
        self.fully_hardware_compatible = self.verify_fully_hardware_compatible(&sub_models);
    }

    fn verify_fully_hardware_compatible(&self, device_sub_models: &[SubModel]) -> bool {
        !device_sub_models
            .iter()
            .any(|sub_model| sub_model.r#type == SubModelType::Software)
    }

    fn get_sub_models(&mut self, device_version: Gna2DeviceVersion) -> Vec<SubModel> {
        self.sub_models
            .entry(device_version)
            .or_insert_with(|| vec![SubModel::new(SubModelType::Hardware, 0)])
            .clone()
    }

    pub fn score_request(&mut self, request_configuration: &RequestConfiguration) -> u32 {
        if self.should_use_software_mode(request_configuration) {
            self.software_model.score()
        } else if let Some(hardware_model) = &mut self.hardware_model {
            hardware_model.score_layer_range(
                0,
                self.software_model.operation_count(),
                request_configuration,
            )
        } else {
            self.software_model.score()
        }
    }

    pub fn invalidate_request_config(&mut self, config_id: u32) {
        if let Some(hardware_model) = &mut self.hardware_model {
            hardware_model.invalidate_config(config_id);
        }
    }

    pub fn validate_buffer(&self, request_allocations: &MemoryContainer, memory: &Memory) {
        if let Some(hardware_model) = &self.hardware_model {
            hardware_model.validate_config_buffer(request_allocations, memory);
        }
    }

    pub fn is_fully_hardware_compatible(&self) -> bool {
        self.fully_hardware_compatible
    }

    pub fn compiled_model(&self) -> &CompiledModel {
        &self.compiled_model
    }

    fn should_use_software_mode(&self, config: &RequestConfiguration) -> bool {
        matches!(
            config.get_acceleration_mode(),
            Gna2AccelerationMode::Software
        ) || (matches!(config.get_acceleration_mode(), Gna2AccelerationMode::Auto)
            && self.hardware_model.is_none())
    }
}

impl IScorable for HybridModel {
    fn score(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_rs::gna_api::types::OperationType;
    use crate::gna_rs::gna_lib::request_configuration::RequestConfiguration;

    #[test]
    fn hybrid_model_new_initializes_without_panic() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());

        let detector = AccelerationDetector;
        let hw_caps = HardwareCapabilities;
        let ddi = DriverInterface::new(0);

        let hybrid = HybridModel::new(model, detector, hw_caps, ddi);
        assert!(!hybrid.is_fully_hardware_compatible() || hybrid.hardware_model.is_some());
    }

    #[test]
    fn hybrid_model_scores_software_when_software_mode_is_enforced() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());

        let detector = AccelerationDetector;
        let hw_caps = HardwareCapabilities;
        let ddi = DriverInterface::new(0);
        let mut hybrid = HybridModel::new(model, detector, hw_caps, ddi);

        let mut config = RequestConfiguration::new();
        config.set_acceleration_mode(Gna2AccelerationMode::Software);

        assert_eq!(hybrid.score_request(&config), 0);
    }

    #[test]
    fn hybrid_model_invalidates_request_config_without_panic() {
        let model = Gna2Model::new();
        let detector = AccelerationDetector;
        let hw_caps = HardwareCapabilities;
        let ddi = DriverInterface::new(0);
        let mut hybrid = HybridModel::new(model, detector, hw_caps, ddi);

        hybrid.invalidate_request_config(1);
    }
}
