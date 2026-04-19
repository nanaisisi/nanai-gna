/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Rust skeleton for `gna2-model-api.h` / `gna2-model-impl.h` types.

/// Opaque model handle used in the original API.
#[derive(Debug, Clone, Default)]
pub struct Gna2Model {
    pub operations: Vec<Gna2Operation>,
}

impl Gna2Model {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, op: Gna2Operation) -> u32 {
        self.operations.push(op);
        (self.operations.len() - 1) as u32
    }

    pub fn operation_count(&self) -> u32 {
        self.operations.len() as u32
    }
}

/// Tensor/shape placeholders
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Gna2Shape {
    // dimensions, strides, etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gna2Tensor {
    pub shape: Gna2Shape,
    pub mode: crate::gna_rs::gna_api::types::Gna2TensorMode,
    pub data_type: crate::gna_rs::gna_api::types::Gna2DataType,
}

impl Default for Gna2Tensor {
    fn default() -> Self {
        Self {
            shape: Gna2Shape::default(),
            mode: crate::gna_rs::gna_api::types::Gna2TensorMode::Default,
            data_type: crate::gna_rs::gna_api::types::Gna2DataType::None,
        }
    }
}

/// Operation structure similar to the C API representation used by the wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gna2Operation {
    pub op_type: crate::gna_rs::gna_api::types::OperationType,
    pub number_of_operands: u32,
    pub number_of_parameters: u32,
    pub operands: Vec<Option<Gna2Tensor>>,
    pub parameters: Vec<Option<Vec<u8>>>,
}

impl Default for Gna2Operation {
    fn default() -> Self {
        Self {
            op_type: crate::gna_rs::gna_api::types::OperationType::Copy,
            number_of_operands: 0,
            number_of_parameters: 0,
            operands: vec![],
            parameters: vec![],
        }
    }
}

/// Common operand index constants (re-exported from types.rs for convenience)
pub use crate::gna_rs::gna_api::types::{
    INPUT_OPERAND_INDEX, OUTPUT_OPERAND_INDEX, SCRATCHPAD_OPERAND_INDEX,
};

/// Type aliases commonly used across the port.
pub type ApiModel = Gna2Model;
pub type ApiShape = Gna2Shape;
pub type ApiTensor = Gna2Tensor;

/// Create a model instance
pub fn gna2_model_create() -> Gna2Model {
    Gna2Model::new()
}

/// Add an operation to the model and return its index
pub fn gna2_model_add_operation(model: &mut Gna2Model, op: Gna2Operation) -> u32 {
    model.add_operation(op)
}

/// Compile the model into a `CompiledModel` (software-only placeholder)
pub fn gna2_model_compile(model: &Gna2Model) -> crate::gna_rs::gna_lib::CompiledModel {
    crate::gna_rs::gna_lib::compiled_model::CompiledModel::new(model.clone())
}
