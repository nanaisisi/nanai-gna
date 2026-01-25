//! Kernel argument structures (ported from `KernelArguments.h` and related headers).

use crate::common::BaseAddress;

/// Minimal arguments passed to kernels. Expand as needed when porting concrete kernels.
#[derive(Debug, Clone)]
pub struct KernelArguments {
    /// Pointer to input buffer
    pub input: BaseAddress,
    /// Pointer to output buffer
    pub output: BaseAddress,
    /// Pointer to weights/biases etc (optional)
    pub aux: Option<BaseAddress>,
    /// input width / height / other shape-related metadata
    pub width: usize,
    pub height: usize,
}

impl KernelArguments {
    pub fn new(input: BaseAddress, output: BaseAddress) -> Self {
        Self { input, output, aux: None, width: 0, height: 0 }
    }
}
