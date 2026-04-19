/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::model_api::Gna2Operation;
use crate::gna_rs::gna_api::types::Gna2TensorMode;
use crate::gna_rs::gna_lib::model_wrapper::ModelWrapper;

/// Simplified Rust port of the GNA ExternalBuffer helper.
#[allow(dead_code)]
pub struct ExternalBuffer;

impl ExternalBuffer {
    pub fn is_supported(operation: &Gna2Operation, operand_index: u32) -> bool {
        if !ModelWrapper::has_enabled_operand(operation, operand_index) {
            return false;
        }
        let operand = operation.operands[operand_index as usize].as_ref().unwrap();
        operand.mode == Gna2TensorMode::ExternalBuffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::model_api::Gna2Operation;
    use crate::gna_rs::gna_api::model_api::Gna2Tensor;
    use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode, OperationType};

    #[test]
    fn external_buffer_is_supported_when_operand_mode_external_buffer() {
        let mut operation = Gna2Operation::default();
        operation.number_of_operands = 1;
        operation.operands.push(Some(Gna2Tensor {
            shape: Default::default(),
            mode: Gna2TensorMode::ExternalBuffer,
            data_type: Gna2DataType::Int8,
        }));
        assert!(ExternalBuffer::is_supported(&operation, 0));
    }

    #[test]
    fn external_buffer_is_not_supported_for_disabled_operand() {
        let mut operation = Gna2Operation::default();
        operation.number_of_operands = 1;
        operation.operands.push(Some(Gna2Tensor {
            shape: Default::default(),
            mode: Gna2TensorMode::Disabled,
            data_type: Gna2DataType::Int8,
        }));
        assert!(!ExternalBuffer::is_supported(&operation, 0));
    }
}
