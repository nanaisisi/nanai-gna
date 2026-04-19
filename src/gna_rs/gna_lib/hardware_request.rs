/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA `HardwareRequest` helper.
use crate::gna_rs::gna_api::types::{
    INPUT_OPERAND_INDEX, OUTPUT_OPERAND_INDEX, OperationType, SCRATCHPAD_OPERAND_INDEX,
};
use crate::gna_rs::gna_lib::hardware_layer::HardwareLayer;
use crate::gna_rs::gna_lib::hardware_model::HardwareModel;
use crate::gna_rs::gna_lib::layer_configuration::LayerConfiguration;
use crate::gna_rs::gna_lib::request_configuration::RequestConfiguration;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GnaOperationMode {
    Gmm = 0,
    Xnn = 1,
}

#[derive(Debug, Clone)]
pub struct MemoryPatch {
    pub offset: u32,
    pub value: u32,
    pub size: usize,
}

#[derive(Debug, Default)]
pub struct DriverMemoryObject {
    pub id: u32,
    pub size: u32,
    pub patches: Vec<MemoryPatch>,
}

#[derive(Debug)]
pub struct HardwareRequest {
    pub hw_perf_encoding: u8,
    pub request_config_id: u32,
    pub request_configuration: RequestConfiguration,
    pub submitted: bool,
    pub submit_ready: bool,
    pub mode: GnaOperationMode,
    pub layer_base: u32,
    pub layer_count: u32,
    pub gmm_offset: u32,
    pub gmm_mode_active_list_on: bool,
    pub driver_memory_objects: Vec<DriverMemoryObject>,
    pub gmm_mode_active_lists: BTreeMap<u32, bool>,
    pub calculation_data: Vec<u8>,
}

impl HardwareRequest {
    pub fn new(request_configuration: RequestConfiguration) -> Self {
        let hw_perf_encoding = request_configuration.get_hw_instrumentation_mode();
        let request_config_id = request_configuration.config_id;

        Self {
            hw_perf_encoding,
            request_config_id,
            request_configuration,
            submitted: false,
            submit_ready: false,
            mode: GnaOperationMode::Xnn,
            layer_base: 0,
            layer_count: 0,
            gmm_offset: 0,
            gmm_mode_active_list_on: false,
            driver_memory_objects: vec![DriverMemoryObject::default()],
            gmm_mode_active_lists: BTreeMap::new(),
            calculation_data: Vec::new(),
        }
    }

    pub fn invalidate(&mut self) {
        self.submit_ready = false;
        self.calculation_data.clear();
        self.driver_memory_objects
            .iter_mut()
            .for_each(|obj| obj.patches.clear());
    }

    pub fn invalidate_with_model(&mut self, hw_model: &HardwareModel) {
        self.invalidate();

        let layer_entries: Vec<(u32, LayerConfiguration)> = self
            .request_configuration
            .layer_configurations
            .iter()
            .map(|(&index, config)| (index, config.clone()))
            .collect();

        for (layer_index, layer_configuration) in layer_entries {
            if let Some(hw_layer) = hw_model.try_get_layer(layer_index as usize) {
                self.generate_buffer_patches(hw_model, layer_index, &layer_configuration, hw_layer);
                if self.should_patch_nn_op_type(hw_layer, &layer_configuration) {
                    let nnop_offset = hw_layer.get_ld_nnop_offset();
                    let nnop_value = hw_layer
                        .get_nn_op_type(layer_configuration.has_active_list())
                        .as_u32();
                    self.add_patch(MemoryPatch {
                        offset: nnop_offset,
                        value: nnop_value,
                        size: 1,
                    });
                }
            }
        }
    }

    pub fn update(
        &mut self,
        hw_model: &HardwareModel,
        layer_index: u32,
        layer_count: u32,
        mode: GnaOperationMode,
    ) {
        let hw_layer = hw_model.get_layer(layer_index as usize);
        self.mode = mode;
        self.layer_count = layer_count;
        self.layer_base = hw_layer.get_xnn_descriptor_offset();

        if mode == GnaOperationMode::Gmm {
            self.gmm_offset = hw_layer.get_gmm_descriptor_offset();
            self.update_gmm_mode_active_lists(hw_model, layer_index, layer_count);
            self.gmm_mode_active_list_on = *self
                .gmm_mode_active_lists
                .get(&layer_index)
                .unwrap_or(&false);
        } else {
            self.gmm_offset = 0;
            self.gmm_mode_active_list_on = false;
        }

        self.submit_ready = true;
    }

    pub fn add_patch(&mut self, patch: MemoryPatch) {
        if let Some(driver_object) = self.driver_memory_objects.first_mut() {
            driver_object.patches.push(patch);
        }
    }

    pub fn set_driver_buffer(&mut self, id: u32, size: u32) {
        if let Some(driver_object) = self.driver_memory_objects.first_mut() {
            driver_object.id = id;
            driver_object.size = size;
        }
    }

    pub fn add_driver_memory_object(&mut self, id: u32, size: u32) {
        self.driver_memory_objects.push(DriverMemoryObject {
            id,
            size,
            patches: Vec::new(),
        });
    }

    pub fn calculation_size(&self) -> usize {
        self.calculation_data.len()
    }

    pub fn calculation_data(&self) -> &[u8] {
        &self.calculation_data
    }

    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn is_submitted(&self) -> bool {
        self.submitted
    }

    fn generate_buffer_patches(
        &mut self,
        hw_model: &HardwareModel,
        _layer_index: u32,
        layer_configuration: &LayerConfiguration,
        hw_layer: &HardwareLayer,
    ) {
        for (&operand_index, &address) in layer_configuration.buffers.iter() {
            let buffer_offset = hw_model.get_buffer_offset(address);
            let ld_offset = match operand_index {
                INPUT_OPERAND_INDEX => hw_layer.get_ld_input_offset(),
                OUTPUT_OPERAND_INDEX => hw_layer.get_ld_output_offset(),
                SCRATCHPAD_OPERAND_INDEX => hw_layer.get_ld_intermediate_output_offset(),
                _ => continue,
            };

            self.add_patch(MemoryPatch {
                offset: ld_offset,
                value: buffer_offset,
                size: 4,
            });
        }
    }

    fn should_patch_nn_op_type(
        &self,
        hw_layer: &HardwareLayer,
        layer_configuration: &LayerConfiguration,
    ) -> bool {
        matches!(
            hw_layer.operation(),
            OperationType::FullyConnectedAffine
                | OperationType::ElementWiseAffine
                | OperationType::Gmm
        ) && layer_configuration.has_active_list()
    }

    fn update_gmm_mode_active_lists(
        &mut self,
        hw_model: &HardwareModel,
        layer_index: u32,
        layer_count: u32,
    ) {
        if self.gmm_mode_active_lists.contains_key(&layer_index) {
            return;
        }

        let active = self
            .request_configuration
            .layer_configurations
            .range(layer_index..layer_index.saturating_add(layer_count))
            .any(|(&layer_id, cfg)| {
                cfg.has_active_list()
                    && matches!(
                        hw_model.try_get_layer(layer_id as usize),
                        Some(layer) if layer.operation() == OperationType::Gmm
                    )
            });

        self.gmm_mode_active_lists.insert(layer_index, active);
    }
}

impl GnaOperationMode {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::{DriverMemoryObject, GnaOperationMode, HardwareRequest, MemoryPatch};
    use crate::gna_rs::common::BaseAddress;
    use crate::gna_rs::gna_api::types::{INPUT_OPERAND_INDEX, OUTPUT_OPERAND_INDEX, OperationType};
    use crate::gna_rs::gna_lib::hardware_layer::HardwareLayer;
    use crate::gna_rs::gna_lib::hardware_model::HardwareModel;
    use crate::gna_rs::gna_lib::layer_configuration::LayerConfiguration;
    use crate::gna_rs::gna_lib::request_configuration::RequestConfiguration;

    #[test]
    fn hardware_request_initializes_with_config_id_and_defaults() {
        let config = RequestConfiguration::new();
        let req = HardwareRequest::new(config);

        assert!(req.request_config_id > 0);
        assert_eq!(req.hw_perf_encoding, 0);
        assert!(!req.submitted);
        assert!(!req.submit_ready);
        assert_eq!(req.mode, GnaOperationMode::Xnn);
        assert_eq!(req.layer_base, 0);
        assert_eq!(req.layer_count, 0);
        assert_eq!(req.gmm_offset, 0);
        assert!(!req.gmm_mode_active_list_on);
        assert_eq!(req.driver_memory_objects.len(), 1);
    }

    #[test]
    fn hardware_request_update_sets_offsets_and_ready_state() {
        let mut model = crate::gna_rs::gna_api::model_api::Gna2Model::new();
        model.add_operation(crate::gna_rs::gna_api::model_api::Gna2Operation::default());
        let compiled = crate::gna_rs::gna_lib::compiled_model::CompiledModel::new(model);
        let hardware_model = HardwareModel::new(
            &compiled,
            crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities,
        );

        let mut req = HardwareRequest::new(RequestConfiguration::new());
        req.update(&hardware_model, 0, 1, GnaOperationMode::Xnn);

        assert_eq!(req.layer_base, 0);
        assert_eq!(req.layer_count, 1);
        assert_eq!(req.gmm_offset, 0);
        assert!(!req.gmm_mode_active_list_on);
        assert!(req.submit_ready);
    }

    #[test]
    fn hardware_request_update_for_gmm_sets_gmm_offset() {
        let mut model = crate::gna_rs::gna_api::model_api::Gna2Model::new();
        model.add_operation(crate::gna_rs::gna_api::model_api::Gna2Operation::default());
        let compiled = crate::gna_rs::gna_lib::compiled_model::CompiledModel::new(model);
        let hardware_model = HardwareModel::new(
            &compiled,
            crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities,
        );

        let mut req = HardwareRequest::new(RequestConfiguration::new());
        req.update(&hardware_model, 0, 1, GnaOperationMode::Gmm);

        assert_eq!(req.gmm_offset, 128);
        assert!(!req.gmm_mode_active_list_on);
        assert!(req.submit_ready);
    }

    #[test]
    fn hardware_request_invalidate_clears_patches_and_ready() {
        let mut req = HardwareRequest::new(RequestConfiguration::new());
        req.add_patch(MemoryPatch {
            offset: 16,
            value: 123,
            size: 4,
        });
        assert_eq!(req.driver_memory_objects[0].patches.len(), 1);

        req.invalidate();
        assert!(!req.submit_ready);
        assert!(req.driver_memory_objects[0].patches.is_empty());
    }

    #[test]
    fn hardware_request_invalidate_with_model_generates_layer_patches() {
        let mut model = crate::gna_rs::gna_api::model_api::Gna2Model::new();
        model.add_operation(crate::gna_rs::gna_api::model_api::Gna2Operation::default());
        let compiled = crate::gna_rs::gna_lib::compiled_model::CompiledModel::new(model);
        let hardware_model = HardwareModel::new(
            &compiled,
            crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities,
        );

        let mut config = RequestConfiguration::new();
        let input_addr = BaseAddress::from_ptr(0x1000usize as *mut u8);
        let output_addr = BaseAddress::from_ptr(0x2000usize as *mut u8);
        config.add_buffer(0, INPUT_OPERAND_INDEX, input_addr);
        config.add_buffer(0, OUTPUT_OPERAND_INDEX, output_addr);

        let mut req = HardwareRequest::new(config);
        req.invalidate_with_model(&hardware_model);

        assert!(!req.driver_memory_objects[0].patches.is_empty());
        assert_eq!(req.driver_memory_objects[0].patches.len(), 2);
    }

    #[test]
    fn hardware_request_submit_marks_submitted() {
        let mut req = HardwareRequest::new(RequestConfiguration::new());
        assert!(!req.is_submitted());
        req.submit();
        assert!(req.is_submitted());
    }
}
