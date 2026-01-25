//! Rust skeleton for `gna2-model-api.h` / `gna2-model-impl.h` types.

/// Opaque model handle used in the original API.
#[derive(Debug, Clone, Default)]
pub struct Gna2Model {
    // fields will be filled during porting
}

/// Tensor/shape placeholders
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Gna2Shape {
    // dimensions, strides, etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gna2Tensor {
    pub shape: Gna2Shape,
    pub mode: crate::gna_api::types::Gna2TensorMode,
    pub data_type: crate::gna_api::types::Gna2DataType,
}

impl Default for Gna2Tensor {
    fn default() -> Self { Self { shape: Gna2Shape::default(), mode: crate::gna_api::types::Gna2TensorMode::Default, data_type: crate::gna_api::types::Gna2DataType::None } }
}

/// Operation structure similar to the C API representation used by the wrapper.
#[derive(Debug, Clone)]
pub struct Gna2Operation {
    pub op_type: crate::gna_api::types::OperationType,
    pub number_of_operands: u32,
    pub number_of_parameters: u32,
    pub operands: Vec<Option<Gna2Tensor>>,
    pub parameters: Vec<Option<Vec<u8>>>,
}

impl Default for Gna2Operation {
    fn default() -> Self { Self { op_type: crate::gna_api::types::OperationType::Copy, number_of_operands: 0, number_of_parameters: 0, operands: vec![], parameters: vec![] } }
}

/// Common operand index constants (re-exported from types.rs for convenience)
pub use crate::gna_api::types::{INPUT_OPERAND_INDEX, OUTPUT_OPERAND_INDEX, SCRATCHPAD_OPERAND_INDEX};

/// Type aliases commonly used across the port.
pub type ApiModel = Gna2Model;
pub type ApiShape = Gna2Shape;
pub type ApiTensor = Gna2Tensor;
