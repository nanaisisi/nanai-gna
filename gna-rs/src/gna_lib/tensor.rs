/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::common::BaseAddress;
use crate::gna_api::model_api::ApiTensor;
use crate::gna_api::types::Gna2TensorMode;
use crate::gna_lib::component::Component;
use crate::gna_lib::data_mode::DataMode;
use crate::gna_lib::shape::Shape;
use crate::gna_lib::validator::Validator;

/// Simplified Rust port of the GNA Tensor helper.
#[derive(Debug, Clone)]
pub struct Tensor {
    pub component: Component,
    pub mode: DataMode,
    pub size: u32,
    pub buffer: BaseAddress,
    pub operand_index: u32,
}

impl Tensor {
    pub fn new(
        dimensions: Shape,
        data_mode: DataMode,
        buffer: BaseAddress,
        operand_index: u32,
    ) -> Self {
        let count = dimensions.get_number_of_elements() as u32;
        let component = Component::new(dimensions, operand_index, false);
        let size = Self::get_effective_size(&data_mode, count);
        Self {
            component,
            mode: data_mode,
            size,
            buffer,
            operand_index,
        }
    }

    pub fn with_validator(
        dimensions: Shape,
        data_mode: DataMode,
        buffer: BaseAddress,
        validator: Validator,
        operand_index: u32,
    ) -> Self {
        let count = dimensions.0.iter().product::<usize>() as u32;
        let component =
            Component::from_validator(dimensions, validator, false, operand_index, false);
        let size = Self::get_effective_size(&data_mode, count);
        let tensor = Self {
            component,
            mode: data_mode,
            size,
            buffer,
            operand_index,
        };
        tensor.validate();
        tensor
    }

    pub fn update_buffer(&mut self, buffer: BaseAddress) -> bool {
        if self.validate_buffer(buffer) {
            self.buffer = buffer;
            true
        } else {
            false
        }
    }

    pub fn validate_buffer(&self, buffer: BaseAddress) -> bool {
        if let Some(validator) = &self.component.validator {
            validator.validate_buffer(buffer.get::<u8>() as *const u8, self.size as usize, 8)
        } else if self.mode.mode == Gna2TensorMode::Disabled {
            buffer.is_null()
        } else {
            true
        }
    }

    pub fn validate(&self) -> bool {
        if self.mode.mode == Gna2TensorMode::Disabled {
            self.buffer.is_null()
        } else if let Some(_) = &self.component.validator {
            self.validate_buffer(self.buffer)
        } else {
            true
        }
    }

    pub fn buffer_address(&self) -> BaseAddress {
        self.buffer
    }

    pub fn dimensions(&self) -> &Shape {
        &self.component.dimensions
    }

    pub fn is_disabled(&self) -> bool {
        self.mode.mode == Gna2TensorMode::Disabled
    }

    pub fn as_api_tensor(&self) -> ApiTensor {
        ApiTensor {
            shape: Default::default(),
            mode: self.mode.mode,
            data_type: self.mode.data_type,
        }
    }

    pub fn get_data_mode(tensor: &ApiTensor) -> DataMode {
        DataMode::with_mode(tensor.data_type, tensor.mode)
    }

    pub fn get_effective_size(mode: &DataMode, count: u32) -> u32 {
        if mode.mode == Gna2TensorMode::ConstantScalar {
            mode.size as u32
        } else {
            count * mode.size as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BaseAddress;
    use crate::gna_api::types::{Gna2DataType, Gna2TensorMode};

    fn default_buffer_validator(buffer: *const u8, size: usize, alignment: u32) -> bool {
        if buffer.is_null() {
            false
        } else {
            let address = buffer as usize;
            address % (alignment as usize) == 0 && size > 0
        }
    }

    #[test]
    fn tensor_size_is_based_on_dimensions_and_data_mode() {
        let shape = Shape::with_dims(vec![2, 3, 4]);
        let mode = DataMode::new(Gna2DataType::Int16);
        let tensor = Tensor::new(shape, mode, BaseAddress::null(), 0);
        assert_eq!(tensor.size, 2 * 3 * 4 * 2);
    }

    #[test]
    fn tensor_update_buffer_accepts_valid_data() {
        let shape = Shape::with_dims(vec![1, 1]);
        let mode = DataMode::new(Gna2DataType::Int16);
        let data = [0u8; 8];
        let base = BaseAddress::from_ptr(data.as_ptr() as *mut u8);

        let base_validator = crate::gna_lib::validator::BaseValidator::new(
            0,
            std::sync::Arc::new(default_buffer_validator),
        );
        let layer_validator = crate::gna_lib::validator::LayerValidator::new(&base_validator, 0);
        let validator = Validator::new(&layer_validator, None, false);
        let mut tensor = Tensor::with_validator(shape, mode, BaseAddress::null(), validator, 1);
        assert!(tensor.update_buffer(base));
        assert_eq!(tensor.buffer, base);
        assert!(tensor.validate());
        assert!(tensor.validate_buffer(base));
        assert!(!tensor.validate_buffer(BaseAddress::null()));
    }

    #[test]
    fn tensor_validate_disabled_requires_null_buffer() {
        let shape = Shape::with_dims(vec![1]);
        let mode = DataMode::with_mode(Gna2DataType::Int16, Gna2TensorMode::Disabled);
        let tensor = Tensor::new(shape, mode, BaseAddress::null(), 2);
        assert!(tensor.validate());
        assert!(tensor.validate_buffer(BaseAddress::null()));
    }
}
