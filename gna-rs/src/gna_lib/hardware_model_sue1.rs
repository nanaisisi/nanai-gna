/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::common::BaseAddress;
use crate::gna_api::gna2_suecreek_header::SueCreekHeader;
use crate::gna_lib::compiled_model::CompiledModel;
use crate::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_lib::hardware_model::HardwareModel;
use crate::gna_lib::layer_descriptor::LayerDescriptor;

/// Simplified Rust port of the GNA `HardwareModelSue1` helper.
#[derive(Debug)]
pub struct HardwareModelSue1 {
    hardware_model: HardwareModel,
    export_memory: Vec<u8>,
    total_model_size: u32,
}

impl HardwareModelSue1 {
    pub fn new(
        software_model: &CompiledModel,
        _custom_alloc: Option<fn(usize) -> *mut u8>,
    ) -> Self {
        let mut model = Self {
            hardware_model: HardwareModel::new(software_model, HardwareCapabilities),
            export_memory: Vec::new(),
            total_model_size: 0,
        };
        model.prepare_allocations_and_model();
        model
    }

    pub fn get_descriptor(&self, layer_index: usize) -> &LayerDescriptor {
        self.hardware_model.get_layer(layer_index).descriptor()
    }

    pub fn get_output_offset(&self, layer_index: usize) -> u32 {
        let layer = self.hardware_model.get_layer(layer_index);
        layer.get_ld_output_offset() - layer.get_xnn_descriptor_offset()
    }

    pub fn get_input_offset(&self, layer_index: usize) -> u32 {
        let layer = self.hardware_model.get_layer(layer_index);
        layer.get_ld_input_offset() - layer.get_xnn_descriptor_offset()
    }

    pub fn get_buffer_offset(&self, address: BaseAddress) -> u32 {
        self.hardware_model.get_buffer_offset(address)
    }

    pub fn export(&mut self) -> *mut u8 {
        self.export_memory.as_mut_ptr()
    }

    pub fn populate_header(&self, model_header: &mut SueCreekHeader) {
        model_header.magic = self.total_model_size;
    }

    fn prepare_allocations_and_model(&mut self) {
        let ld_memory_size = self.hardware_model.calculate_descriptor_size(false) as usize;
        self.export_memory.resize(ld_memory_size, 0);
        self.total_model_size = ld_memory_size as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_api::types::OperationType;

    #[test]
    fn hardware_model_sue1_new_allocates_export_memory() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let mut sue1 = HardwareModelSue1::new(&compiled, None);
        assert!(!sue1.export().is_null());
        assert_eq!(sue1.total_model_size, sue1.export_memory.len() as u32);
    }

    #[test]
    fn hardware_model_sue1_offsets_are_computed_relative_to_descriptor() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let sue1 = HardwareModelSue1::new(&compiled, None);
        let descriptor = sue1.get_descriptor(0);
        assert_eq!(descriptor.layer_index(), 0);
        assert!(sue1.get_input_offset(0) >= 0);
        assert!(sue1.get_output_offset(0) > 0);
    }

    #[test]
    fn hardware_model_sue1_populate_header_sets_magic() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let sue1 = HardwareModelSue1::new(&compiled, None);
        let mut header = SueCreekHeader::default();
        sue1.populate_header(&mut header);

        assert_eq!(header.magic, sue1.total_model_size);
    }
}
