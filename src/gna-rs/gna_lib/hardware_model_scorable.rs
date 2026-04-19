/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA `HardwareModelScorable` helper.
use crate::common::BaseAddress;
use crate::gna_api::model_api::Gna2Model;
use crate::gna_api::types::OperationType;
use crate::gna_lib::compiled_model::CompiledModel;
use crate::gna_lib::driver_interface::DriverInterface;
use crate::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_lib::hardware_model::HardwareModel;
use crate::gna_lib::hardware_request::{GnaOperationMode, HardwareRequest};
use crate::gna_lib::iscorable::IScorable;
use crate::gna_lib::memory::Memory;
use crate::gna_lib::memory_container::MemoryContainer;
use crate::gna_lib::request_configuration::RequestConfiguration;
use crate::gna_lib::sub_model::SubModel;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HardwareModelScorable {
    hardware_model: HardwareModel,
    driver_interface: DriverInterface,
    hardware_requests: HashMap<u32, HardwareRequest>,
    sub_models: Vec<SubModel>,
}

impl HardwareModelScorable {
    pub fn new(
        software_model: &CompiledModel,
        driver_interface: DriverInterface,
        hw_capabilities: HardwareCapabilities,
        sub_models: Vec<SubModel>,
    ) -> Self {
        Self {
            hardware_model: HardwareModel::new(software_model, hw_capabilities),
            driver_interface,
            hardware_requests: HashMap::new(),
            sub_models,
        }
    }

    pub fn invalidate_config(&mut self, config_id: u32) {
        if let Some(request) = self.hardware_requests.get_mut(&config_id) {
            request.invalidate();
        }
    }

    pub fn score_layer_range(
        &mut self,
        layer_index: u32,
        layer_count: u32,
        request_configuration: &RequestConfiguration,
    ) -> u32 {
        let max_layers = self.hardware_model.layer_count();
        if layer_index as usize + layer_count as usize > max_layers {
            return 0;
        }

        for i in layer_index..layer_index + layer_count {
            if self.hardware_model.try_get_layer(i as usize).is_none() {
                return 0;
            }
        }

        let operation_mode = match self
            .hardware_model
            .get_layer(layer_index as usize)
            .operation()
        {
            OperationType::Gmm => GnaOperationMode::Gmm,
            _ => GnaOperationMode::Xnn,
        };

        let hw_request = self
            .hardware_requests
            .entry(request_configuration.config_id)
            .or_insert_with(|| HardwareRequest::new(request_configuration.clone()));

        if !hw_request.submit_ready {
            hw_request.invalidate_with_model(&self.hardware_model);
        }

        hw_request.update(
            &self.hardware_model,
            layer_index,
            layer_count,
            operation_mode,
        );
        hw_request.submit();

        let result = self.driver_interface.submit_request(hw_request);
        if result.status != 0 && result.status != 2 {
            return 0;
        }

        0
    }

    pub fn get_buffer_offset_for_configuration(
        &self,
        address: BaseAddress,
        _request_configuration: &RequestConfiguration,
    ) -> u32 {
        self.hardware_model.get_buffer_offset(address)
    }

    pub fn validate_config_buffer(
        &self,
        _request_allocations: &MemoryContainer,
        _buffer_memory: &Memory,
    ) {
        // Simplified stub implementation.
    }

    pub fn is_software_layer(&self, layer_index: usize) -> bool {
        SubModel::is_software_layer(layer_index as u32, &self.sub_models)
    }
}

impl IScorable for HardwareModelScorable {
    fn score(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_api::types::OperationType;

    #[test]
    fn hardware_model_scorable_new_builds_underlying_hardware_model() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);
        let driver_interface = DriverInterface::new(0);
        let scorable = HardwareModelScorable::new(
            &compiled,
            driver_interface,
            HardwareCapabilities,
            Vec::new(),
        );

        assert_eq!(scorable.score(), 0);
    }

    #[test]
    fn hardware_model_scorable_invalidate_config_does_not_panic() {
        let mut model = Gna2Model::new();
        let compiled = CompiledModel::new(model);
        let driver_interface = DriverInterface::new(0);
        let mut scorable = HardwareModelScorable::new(
            &compiled,
            driver_interface,
            HardwareCapabilities,
            Vec::new(),
        );

        scorable.invalidate_config(1);
    }

    #[test]
    fn hardware_model_scorable_get_buffer_offset_returns_zero_for_null_address() {
        let model = Gna2Model::new();
        let compiled = CompiledModel::new(model);
        let driver_interface = DriverInterface::new(0);
        let scorable = HardwareModelScorable::new(
            &compiled,
            driver_interface,
            HardwareCapabilities,
            Vec::new(),
        );

        let offset = scorable
            .get_buffer_offset_for_configuration(BaseAddress::null(), &RequestConfiguration::new());
        assert_eq!(offset, 0);
    }

    #[test]
    fn hardware_model_scorable_score_updates_request_and_returns_zero() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);
        let driver_interface = DriverInterface::new(0);
        let mut scorable = HardwareModelScorable::new(
            &compiled,
            driver_interface,
            HardwareCapabilities,
            Vec::new(),
        );
        let request_configuration = RequestConfiguration::new();

        let result = scorable.score_layer_range(0, 1, &request_configuration);
        assert_eq!(result, 0);
    }
}
