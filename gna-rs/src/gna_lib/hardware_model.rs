/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::common::BaseAddress;
use crate::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_api::types::OperationType;
use crate::gna_lib::compiled_model::CompiledModel;
use crate::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_lib::hardware_layer::HardwareLayer;
use crate::gna_lib::layer_descriptor::LayerDescriptor;

/// Simplified Rust port of the GNA `HardwareModel` helper.
#[derive(Debug)]
pub struct HardwareModel {
    software_model: CompiledModel,
    hw_capabilities: HardwareCapabilities,
    gmm_descriptors_size: u32,
    xnn_descriptors_size: u32,
    hardware_layers: Vec<HardwareLayer>,
    base_address: BaseAddress,
}

impl HardwareModel {
    pub fn new(software_model: &CompiledModel, hw_capabilities: HardwareCapabilities) -> Self {
        let gmm_count = software_model
            .model
            .operations
            .iter()
            .filter(|op| op.op_type == OperationType::Gmm)
            .count() as u32;
        let layer_count = software_model.model.operation_count();
        let device_version = hw_capabilities.get_device_version();

        let xnn_descriptors_size = Self::get_layer_descriptors_size(layer_count, device_version);
        let gmm_descriptors_size = Self::get_gmm_descriptors_size(gmm_count);

        let mut model = Self {
            software_model: software_model.clone(),
            hw_capabilities,
            gmm_descriptors_size,
            xnn_descriptors_size,
            hardware_layers: Vec::new(),
            base_address: BaseAddress::null(),
        };

        model.build();
        model
    }

    pub fn build(&mut self) {
        self.hardware_layers = self
            .software_model
            .model
            .operations
            .iter()
            .enumerate()
            .map(|(index, op)| {
                let mut descriptor = LayerDescriptor::new(index, op.op_type, op.operands.len(), 1);

                descriptor.set_parameter("xnn_descriptor_offset", (index as u32) * 64);
                descriptor.set_parameter("in_buffer", (index as u32) * 64 + 0x10);
                descriptor.set_parameter("out_buffer", (index as u32) * 64 + 0x20);
                descriptor.set_parameter("weight_buffer", (index as u32) * 64 + 0x30);
                descriptor.set_parameter("bias_buffer", (index as u32) * 64 + 0x40);
                descriptor.set_parameter("pwl_seg_def_buffer", (index as u32) * 64 + 0x50);
                descriptor.set_parameter("act_list_buffer", (index as u32) * 64 + 0x60);
                descriptor.set_parameter("act_list_n_elems", (index as u32) * 64 + 0x70);
                descriptor.set_parameter("gmm_descriptor", (index as u32) * 16 + 0x80);
                descriptor.set_parameter("gmmscrlen", (index as u32) * 16 + 0x90);

                HardwareLayer::new(op.op_type, descriptor, false)
            })
            .collect();
    }

    pub fn layer_count(&self) -> usize {
        self.hardware_layers.len()
    }

    pub fn get_layer(&self, layer_index: usize) -> &HardwareLayer {
        self.hardware_layers
            .get(layer_index)
            .expect("layer index out of bounds")
    }

    pub fn try_get_layer(&self, layer_index: usize) -> Option<&HardwareLayer> {
        self.hardware_layers.get(layer_index)
    }

    pub fn calculate_descriptor_size(&self, include_gmms: bool) -> u32 {
        self.xnn_descriptors_size
            + if include_gmms {
                self.gmm_descriptors_size
            } else {
                0
            }
    }

    pub fn get_buffer_offset(&self, address: BaseAddress) -> u32 {
        address.get_offset(&self.base_address)
    }

    pub fn get_layer_descriptors_size(layer_count: u32, _device_version: Gna2DeviceVersion) -> u32 {
        layer_count * 64
    }

    pub fn get_gmm_descriptors_size(gmm_count: u32) -> u32 {
        gmm_count * 16
    }

    pub fn is_software_layer(&self, _layer_index: usize) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{HardwareCapabilities, HardwareModel};
    use crate::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_api::types::OperationType;

    #[test]
    fn hardware_model_builds_layers_for_compiled_model() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        model.add_operation(Gna2Operation {
            op_type: OperationType::Gmm,
            number_of_operands: 0,
            number_of_parameters: 0,
            operands: vec![],
            parameters: vec![],
        });

        let compiled = crate::gna_lib::compiled_model::CompiledModel::new(model);
        let hardware_model = HardwareModel::new(&compiled, HardwareCapabilities);

        assert_eq!(hardware_model.layer_count(), 2);
        assert!(hardware_model.try_get_layer(0).is_some());
        assert!(hardware_model.try_get_layer(2).is_none());
        assert_eq!(hardware_model.calculate_descriptor_size(true), 64 * 2 + 16);
        assert_eq!(hardware_model.calculate_descriptor_size(false), 64 * 2);
    }

    #[test]
    fn hardware_model_gets_buffer_offset_from_base_address() {
        let model = Gna2Model::new();
        let compiled = crate::gna_lib::compiled_model::CompiledModel::new(model);
        let hardware_model = HardwareModel::new(&compiled, HardwareCapabilities);

        let offset = hardware_model.get_buffer_offset(crate::common::BaseAddress::null());
        assert_eq!(offset, 0);
    }
}
