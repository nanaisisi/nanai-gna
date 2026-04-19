/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::gna_lib::active_list::ActiveList;
use crate::gna_rs::gna_lib::buffer_map::BufferMap;
use crate::gna_rs::gna_lib::tensor::Tensor;

/// Skeleton for Transform base types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformOperation {
    Unknown,
    Affine,
    AffineActiveList,
    Gmm,
    GmmActiveList,
    Cnn,
    Rnn,
    Copy,
    Transposition,
}

impl Default for TransformOperation {
    fn default() -> Self {
        TransformOperation::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct BaseTransform {
    operation: TransformOperation,
    input: Option<Tensor>,
    output: Option<Tensor>,
}

impl Default for BaseTransform {
    fn default() -> Self {
        Self {
            operation: TransformOperation::Unknown,
            input: None,
            output: None,
        }
    }
}

impl BaseTransform {
    pub fn new(operation: TransformOperation) -> Self {
        Self {
            operation,
            input: None,
            output: None,
        }
    }

    pub fn with_io(operation: TransformOperation, input: Tensor, output: Tensor) -> Self {
        Self {
            operation,
            input: Some(input),
            output: Some(output),
        }
    }

    pub fn operation(&self) -> TransformOperation {
        self.operation
    }

    pub fn input(&self) -> Option<&Tensor> {
        self.input.as_ref()
    }

    pub fn output(&self) -> Option<&Tensor> {
        self.output.as_ref()
    }

    pub fn compute(&self) {
        // placeholder
    }

    pub fn update_config_buffers(&self, _buffers: &BufferMap) {
        // placeholder: no config object support in skeleton
    }

    pub fn validate_active_list(&self, _active_list: &ActiveList) -> bool {
        false
    }

    pub fn set_output(&mut self, output_buffer: BaseAddress) -> bool {
        if let Some(output) = &mut self.output {
            output.update_buffer(output_buffer)
        } else {
            false
        }
    }

    pub fn get_operand(&self, operand_index: u32) -> Option<&Tensor> {
        match operand_index {
            0 => self.input.as_ref(),
            1 => self.output.as_ref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::common::BaseAddress;
    use crate::gna_rs::gna_api::types::Gna2DataType;
    use crate::gna_rs::gna_lib::data_mode::DataMode;

    #[test]
    fn base_transform_can_store_input_and_output() {
        let input = Tensor::new(
            crate::gna_rs::gna_lib::shape::Shape::with_dims(vec![1, 2]),
            DataMode::new(Gna2DataType::Int16),
            BaseAddress::null(),
            0,
        );
        let output = Tensor::new(
            crate::gna_rs::gna_lib::shape::Shape::with_dims(vec![1, 1]),
            DataMode::new(Gna2DataType::Int16),
            BaseAddress::null(),
            1,
        );

        let transform =
            BaseTransform::with_io(TransformOperation::Affine, input.clone(), output.clone());

        assert_eq!(transform.operation(), TransformOperation::Affine);
        assert_eq!(transform.input().unwrap().operand_index, 0);
        assert_eq!(transform.output().unwrap().operand_index, 1);
    }

    #[test]
    fn base_transform_can_set_output_buffer() {
        let input = Tensor::new(
            crate::gna_rs::gna_lib::shape::Shape::with_dims(vec![1]),
            DataMode::new(Gna2DataType::Int16),
            BaseAddress::null(),
            0,
        );
        let mut output = Tensor::new(
            crate::gna_rs::gna_lib::shape::Shape::with_dims(vec![1]),
            DataMode::new(Gna2DataType::Int16),
            BaseAddress::null(),
            1,
        );
        let mut transform = BaseTransform::with_io(TransformOperation::Copy, input, output);

        let addr = BaseAddress::from_ptr(0x1000usize as *mut u8);
        assert!(transform.set_output(addr));
        assert_eq!(transform.output().unwrap().buffer_address(), addr);
    }

    #[test]
    fn base_transform_get_operand_returns_none_for_invalid_index() {
        let transform = BaseTransform::new(TransformOperation::Copy);
        assert!(transform.get_operand(10).is_none());
    }
}
