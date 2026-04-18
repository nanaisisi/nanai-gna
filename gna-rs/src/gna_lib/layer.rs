/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Simplified Rust port of the GNA `Layer` helper.
use crate::common::BaseAddress;
use crate::gna_lib::buffer_map::BufferMap;
use crate::gna_lib::layer_configuration::LayerConfiguration;
use crate::gna_lib::layer_input::LayerInput;
use crate::gna_lib::layer_output::LayerOutput;
use crate::gna_lib::transform::TransformOperation;
use crate::gna_lib::{BaseTransform, TransformMap};

#[derive(Debug, Clone)]
pub struct Layer {
    pub buffers: BufferMap,
    pub input: LayerInput,
    pub output: LayerOutput,
    pub transforms: TransformMap,
    pub input_transform: Option<TransformOperation>,
    pub output_transform: Option<TransformOperation>,
    pub has_1b_input_and_2b_weight: bool,
    pub is_1b_input_and_2b_weight_verified: bool,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            buffers: BufferMap::new(),
            input: LayerInput::new(Vec::new(), 0, 0),
            output: LayerOutput::new(0, 0),
            transforms: TransformMap::new(),
            input_transform: None,
            output_transform: None,
            has_1b_input_and_2b_weight: false,
            is_1b_input_and_2b_weight_verified: false,
        }
    }

    pub fn with_buffers(buffers: BufferMap) -> Self {
        Self {
            buffers,
            input: LayerInput::new(Vec::new(), 0, 0),
            output: LayerOutput::new(0, 0),
            transforms: TransformMap::new(),
            input_transform: None,
            output_transform: None,
            has_1b_input_and_2b_weight: false,
            is_1b_input_and_2b_weight_verified: false,
        }
    }

    pub fn with_io(input: LayerInput, output: LayerOutput) -> Self {
        Self {
            buffers: BufferMap::new(),
            input,
            output,
            transforms: TransformMap::new(),
            input_transform: None,
            output_transform: None,
            has_1b_input_and_2b_weight: false,
            is_1b_input_and_2b_weight_verified: false,
        }
    }

    pub fn set_buffer(&mut self, operand_index: u32, address: BaseAddress) {
        self.buffers.insert(operand_index, address);
    }

    pub fn get_buffer(&self, operand_index: u32) -> Option<BaseAddress> {
        self.buffers.get(operand_index)
    }

    pub fn has_buffer(&self, operand_index: u32) -> bool {
        self.buffers.get(operand_index).is_some()
    }

    pub fn clear_buffers(&mut self) {
        self.buffers = BufferMap::new();
    }

    pub fn add_transform(&mut self, transform: BaseTransform) {
        let operation = transform.operation();
        self.transforms.emplace(transform);
        if self.input_transform.is_none() {
            self.input_transform = Some(operation);
        }
        self.output_transform = Some(operation);
    }

    pub fn init_transforms(&mut self, operations: &[TransformOperation]) {
        self.transforms = TransformMap::new();
        self.input_transform = None;
        self.output_transform = None;

        for op in operations {
            let transform = BaseTransform::new(*op);
            self.add_transform(transform);
        }
    }

    pub fn clear_transforms(&mut self) {
        self.transforms = TransformMap::new();
        self.input_transform = None;
        self.output_transform = None;
    }

    pub fn compute(&self, layer_configuration: Option<&LayerConfiguration>) {
        for transform in self.transforms.iter() {
            transform.compute();
            if let Some(config) = layer_configuration {
                transform.update_config_buffers(&config.buffers);
                if let Some(active_list) = config.get_active_list() {
                    transform.validate_active_list(active_list);
                }
            }
        }
    }

    pub fn get_transform_operand(
        &self,
        operation: TransformOperation,
        operand_index: u32,
    ) -> Option<&crate::gna_lib::tensor::Tensor> {
        self.transforms
            .get_optional(operation)
            .and_then(|transform| transform.get_operand(operand_index))
    }

    pub fn verify_has_1b_input_and_2b_weight(&mut self) {
        self.is_1b_input_and_2b_weight_verified = true;
        self.has_1b_input_and_2b_weight = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BaseAddress;

    #[test]
    fn layer_can_store_and_retrieve_buffers() {
        let mut layer = Layer::new();
        let addr = BaseAddress::from_ptr(0x1000usize as *mut u8);

        layer.set_buffer(0, addr);
        assert_eq!(layer.get_buffer(0), Some(addr));
        assert!(layer.has_buffer(0));
        assert!(!layer.has_buffer(1));
    }

    #[test]
    fn layer_can_clear_buffers() {
        let mut layer = Layer::new();
        let addr = BaseAddress::from_ptr(0x2000usize as *mut u8);
        layer.set_buffer(1, addr);

        layer.clear_buffers();
        assert!(!layer.has_buffer(1));
    }

    #[test]
    fn layer_can_be_created_with_io() {
        let input = LayerInput::from_bytes(&[1, 2, 3]);
        let output = LayerOutput::new(2, 3);
        let layer = Layer::with_io(input.clone(), output.clone());

        assert_eq!(layer.input.read(), input.read());
        assert_eq!(layer.output.grouping(), output.grouping());
    }

    #[test]
    fn layer_can_manage_transforms() {
        let mut layer = Layer::new();
        layer.add_transform(BaseTransform::default());
        assert_eq!(layer.transforms.len(), 1);
        layer.clear_transforms();
        assert!(layer.transforms.is_empty());
    }

    #[test]
    fn layer_can_verify_1b_input_and_2b_weight() {
        let mut layer = Layer::new();
        assert!(!layer.is_1b_input_and_2b_weight_verified);
        layer.verify_has_1b_input_and_2b_weight();
        assert!(layer.is_1b_input_and_2b_weight_verified);
        assert!(layer.has_1b_input_and_2b_weight);
    }
}
