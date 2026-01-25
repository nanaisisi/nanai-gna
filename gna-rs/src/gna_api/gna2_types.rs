//! Minimal type aliases and enums used by the ported code.

/// Example: device index type
pub type DeviceIndex = u32;

/// Operand index constants (from gna2-model-impl.h)
pub const SCRATCHPAD_OPERAND_INDEX: u32 = u32::MAX;
pub const INPUT_OPERAND_INDEX: u32 = 0;
pub const OUTPUT_OPERAND_INDEX: u32 = 1;
pub const WEIGHT_OPERAND_INDEX: u32 = 2;
pub const BIAS_OPERAND_INDEX: u32 = 3;
pub const PWL_OPERAND_INDEX: u32 = 4;

/// Enum describing operation types used by GNA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationType {
    FullyConnectedAffine,
    ElementWiseAffine,
    Recurrent,
    Copy,
    Convolution,
    Gmm,
    Transposition,
}

/// Tensor modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2TensorMode {
    Default,
    Disabled,
    ExternalBuffer,
}

/// Data types (partial)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2DataType {
    None,
    Int8,
    Int16,
    Int32,
}
