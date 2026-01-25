/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// ModelWrapper utilities ported from the original GNA implementation.

use crate::gna_api::model_api::{Gna2Operation, Gna2Tensor};
use crate::gna_api::types::{OperationType, Gna2TensorMode};
use std::collections::HashMap;

pub struct ModelWrapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationInfoKey {
    NumberOfOperandsMax,
    NumberOfOperandsRequired,
    NumberOfParametersMax,
    NumberOfParametersRequired,
}

impl ModelWrapper {
    /// Initialize operation operands/parameters vectors
    pub fn operation_init(op: &mut Gna2Operation, op_type: OperationType, _init_only_required_operands: bool) {
        let number_of_operands = Self::get_operation_info(op_type, OperationInfoKey::NumberOfOperandsMax);
        let number_of_parameters = Self::get_operation_info(op_type, OperationInfoKey::NumberOfParametersMax);
        op.op_type = op_type;
        op.number_of_operands = number_of_operands;
        op.number_of_parameters = number_of_parameters;
        op.operands = vec![None; number_of_operands as usize];
        op.parameters = vec![None; number_of_parameters as usize];
    }

    pub fn get_operation_info(op: OperationType, key: OperationInfoKey) -> u32 {
        use OperationType::*;
        match op {
            Copy => match key {
                OperationInfoKey::NumberOfOperandsMax => 2,
                OperationInfoKey::NumberOfOperandsRequired => 2,
                OperationInfoKey::NumberOfParametersMax => 1,
                OperationInfoKey::NumberOfParametersRequired => 1,
            },
            Convolution => match key {
                OperationInfoKey::NumberOfOperandsMax => 5,
                OperationInfoKey::NumberOfOperandsRequired => 3,
                OperationInfoKey::NumberOfParametersMax => 6,
                OperationInfoKey::NumberOfParametersRequired => 1,
            },
            ElementWiseAffine => match key {
                OperationInfoKey::NumberOfOperandsMax => 5,
                OperationInfoKey::NumberOfOperandsRequired => 4,
                OperationInfoKey::NumberOfParametersMax => 0,
                OperationInfoKey::NumberOfParametersRequired => 0,
            },
            FullyConnectedAffine => match key {
                OperationInfoKey::NumberOfOperandsMax => 6,
                OperationInfoKey::NumberOfOperandsRequired => 4,
                OperationInfoKey::NumberOfParametersMax => 2,
                OperationInfoKey::NumberOfParametersRequired => 0,
            },
            Gmm => match key {
                OperationInfoKey::NumberOfOperandsMax => 5,
                OperationInfoKey::NumberOfOperandsRequired => 3,
                OperationInfoKey::NumberOfParametersMax => 1,
                OperationInfoKey::NumberOfParametersRequired => 1,
            },
            Recurrent => match key {
                OperationInfoKey::NumberOfOperandsMax => 5,
                OperationInfoKey::NumberOfOperandsRequired => 5,
                OperationInfoKey::NumberOfParametersMax => 1,
                OperationInfoKey::NumberOfParametersRequired => 1,
            },
            Transposition => match key {
                OperationInfoKey::NumberOfOperandsMax => 2,
                OperationInfoKey::NumberOfOperandsRequired => 2,
                OperationInfoKey::NumberOfParametersMax => 0,
                OperationInfoKey::NumberOfParametersRequired => 0,
            },
        }
    }

    pub fn has_enabled_operand(api_op: &Gna2Operation, operand_index: u32) -> bool {
        (api_op.number_of_operands > operand_index)
            && (api_op.operands.get(operand_index as usize).is_some())
            && (api_op.operands[operand_index as usize].is_some())
            && (api_op.operands[operand_index as usize].as_ref().unwrap().mode != Gna2TensorMode::Disabled)
    }

    pub fn is_operand_available(api_op: &Gna2Operation, index: usize) -> bool {
        api_op.operands.get(index).is_some() && api_op.operands[index].is_some()
    }

    pub fn get_operand(api_op: &Gna2Operation, index: usize, default: Gna2Tensor) -> Gna2Tensor {
        if Self::is_operand_available(api_op, index) {
            api_op.operands[index].clone().unwrap()
        } else {
            default
        }
    }

    pub fn get_enabled_operand(api_op: &Gna2Operation, operand_index: usize) -> Option<Gna2Tensor> {
        if api_op.operands.get(operand_index).is_none() { return None; }
        let operand = api_op.operands[operand_index].clone().unwrap();
        if operand.mode == Gna2TensorMode::Disabled { return None; }
        Some(operand)
    }

    pub fn has_parameter(operation: &Gna2Operation, parameter_index: usize) -> bool {
        operation.parameters.get(parameter_index).is_some() && operation.parameters[parameter_index].is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_api::model_api::Gna2Tensor;

    #[test]
    fn op_info_values() {
        assert_eq!(ModelWrapper::get_operation_info(OperationType::Copy, OperationInfoKey::NumberOfOperandsMax), 2);
        assert_eq!(ModelWrapper::get_operation_info(OperationType::FullyConnectedAffine, OperationInfoKey::NumberOfParametersMax), 2);
    }

    #[test]
    fn operation_init_and_operands() {
        let mut op = Gna2Operation::default();
        ModelWrapper::operation_init(&mut op, OperationType::Copy, true);
        assert_eq!(op.number_of_operands, 2);
        assert_eq!(op.operands.len(), 2);
        op.operands[0] = Some(Gna2Tensor::default());
        assert!(ModelWrapper::is_operand_available(&op, 0));
    }
}
