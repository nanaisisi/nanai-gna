/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_rs::gna_lib::compiled_model::CompiledModel;
use crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_rs::gna_lib::hardware_model::HardwareModel;
use crate::gna_rs::gna_lib::layer_descriptor::LayerDescriptor;
/// Memory container placeholder used by NoMMU buffer validation.
#[derive(Debug, Clone)]
pub struct MemoryContainer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gna2MemoryTag {
    ReadWrite,
    Input,
    Output,
    ReadOnly,
    Scratch,
    State,
    ExternalBufferInput,
    ExternalBufferOutput,
}

impl Gna2MemoryTag {
    pub fn as_u32(self) -> u32 {
        match self {
            Gna2MemoryTag::ReadWrite => 0,
            Gna2MemoryTag::Input => 1,
            Gna2MemoryTag::Output => 2,
            Gna2MemoryTag::ReadOnly => 3,
            Gna2MemoryTag::Scratch => 4,
            Gna2MemoryTag::State => 5,
            Gna2MemoryTag::ExternalBufferInput => 6,
            Gna2MemoryTag::ExternalBufferOutput => 7,
        }
    }
}

impl MemoryContainer {
    pub fn new() -> Self {
        Self {}
    }
}

/// Minimal enum representing export components for NoMMU models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2ModelExportComponent {
    LayerDescriptors,
    ReadOnlyDump,
    InputDump,
    OutputDump,
    ScratchDump,
    StateDump,
}

/// Simplified Rust port of the GNA `HardwareModelNoMMU` helper.
#[derive(Debug)]
pub struct HardwareModelNoMMU {
    hardware_model: HardwareModel,
    target_device: Gna2DeviceVersion,
    export_memory: Vec<u8>,
}

/// Custom allocator callback type.
pub type Gna2UserAllocator = fn(usize) -> *mut u8;

impl HardwareModelNoMMU {
    pub fn new(
        software_model: &CompiledModel,
        _custom_alloc: Option<Gna2UserAllocator>,
        target_device: Gna2DeviceVersion,
    ) -> Self {
        let mut instance = Self {
            hardware_model: HardwareModel::new(software_model, HardwareCapabilities),
            target_device,
            export_memory: Vec::new(),
        };
        instance.prepare_allocations_and_model();
        instance
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

    pub fn export_component(&self, component: Gna2ModelExportComponent) -> Option<&[u8]> {
        match component {
            Gna2ModelExportComponent::LayerDescriptors => Some(&self.export_memory),
            _ => None,
        }
    }

    pub fn set_bar_index(offset_from_bar: u32, bar_index: u32) -> u32 {
        offset_from_bar | bar_index
    }

    pub fn get_bar_index(target_device: Gna2DeviceVersion, tag: Gna2MemoryTag) -> u32 {
        match target_device {
            Gna2DeviceVersion(0x31) => match tag {
                Gna2MemoryTag::ReadWrite => 1,
                Gna2MemoryTag::Input => 2,
                Gna2MemoryTag::Output => 3,
                Gna2MemoryTag::ReadOnly => 1,
                Gna2MemoryTag::Scratch => 0,
                Gna2MemoryTag::State => 3,
                Gna2MemoryTag::ExternalBufferInput => 2,
                Gna2MemoryTag::ExternalBufferOutput => 3,
            },
            _ => panic!("unsupported device version"),
        }
    }

    fn prepare_allocations_and_model(&mut self) {
        let ld_memory_size = self.hardware_model.calculate_descriptor_size(false) as usize;
        self.export_memory.resize(ld_memory_size, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn hardware_model_no_mmu_new_allocates_export_memory() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let no_mmu = HardwareModelNoMMU::new(&compiled, None, Gna2DeviceVersion(0x31));
        assert!(!no_mmu.export_memory.is_empty());
    }

    #[test]
    fn hardware_model_no_mmu_descriptor_access_returns_layer_descriptor() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let no_mmu = HardwareModelNoMMU::new(&compiled, None, Gna2DeviceVersion(0x31));

        let descriptor = no_mmu.get_descriptor(0);
        assert_eq!(descriptor.layer_index(), 0);
    }

    #[test]
    fn hardware_model_no_mmu_get_bar_index_returns_expected_value() {
        assert_eq!(
            HardwareModelNoMMU::get_bar_index(Gna2DeviceVersion(0x31), Gna2MemoryTag::Input),
            2
        );
        assert_eq!(
            HardwareModelNoMMU::get_bar_index(Gna2DeviceVersion(0x31), Gna2MemoryTag::Output),
            3
        );
    }

    #[test]
    fn hardware_model_no_mmu_export_component_returns_layer_descriptors() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let no_mmu = HardwareModelNoMMU::new(&compiled, None, Gna2DeviceVersion(0x31));
        let data = no_mmu.export_component(Gna2ModelExportComponent::LayerDescriptors);
        assert!(data.is_some());
    }
}
