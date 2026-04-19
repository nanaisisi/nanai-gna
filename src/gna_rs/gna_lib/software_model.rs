/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::inference_api::Gna2AccelerationMode;
use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
use crate::gna_rs::gna_lib::iscorable::IScorable;
use std::collections::BTreeMap;

/// Simplified Rust port of the GNA `SoftwareModel` helper.
#[derive(Debug, Clone)]
pub struct SoftwareModel {
    pub model: Gna2Model,
    supported_cpu_accelerations: Vec<Gna2AccelerationMode>,
    layer_count: u32,
    maximum_operand_sizes: BTreeMap<u32, u32>,
}

impl SoftwareModel {
    pub fn new(model: Gna2Model) -> Self {
        Self {
            layer_count: model.operation_count(),
            supported_cpu_accelerations: Vec::new(),
            maximum_operand_sizes: BTreeMap::new(),
            model,
        }
    }

    pub fn with_supported_accelerations(
        model: Gna2Model,
        supported_cpu_accelerations: Vec<Gna2AccelerationMode>,
    ) -> Self {
        Self {
            layer_count: model.operation_count(),
            supported_cpu_accelerations,
            maximum_operand_sizes: BTreeMap::new(),
            model,
        }
    }

    pub fn operation_count(&self) -> u32 {
        self.layer_count
    }

    pub fn get_operation(&self, op_index: usize) -> Option<&Gna2Operation> {
        self.model.operations.get(op_index)
    }

    pub fn get_operations(&self) -> &[Gna2Operation] {
        &self.model.operations
    }

    pub fn supported_accelerations(&self) -> &[Gna2AccelerationMode] {
        &self.supported_cpu_accelerations
    }

    pub fn get_maximum_operand_size(&mut self, operand_index: u32) -> u32 {
        if let Some(&size) = self.maximum_operand_sizes.get(&operand_index) {
            return size;
        }

        let max_size = self.find_maximum_operand_size(operand_index);
        self.maximum_operand_sizes.insert(operand_index, max_size);
        max_size
    }

    fn find_maximum_operand_size(&self, operand_index: u32) -> u32 {
        self.model
            .operations
            .iter()
            .filter_map(|op| op.operands.get(operand_index as usize))
            .map(|operand| operand.as_ref())
            .filter_map(|tensor| tensor.map(|_| 1u32))
            .max()
            .unwrap_or(0)
    }
}

impl IScorable for SoftwareModel {
    fn score(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn software_model_new_creates_instance() {
        let model = Gna2Model::new();
        let software_model = SoftwareModel::new(model);

        assert_eq!(software_model.operation_count(), 0);
        assert!(software_model.supported_accelerations().is_empty());
    }

    #[test]
    fn software_model_returns_operations_and_counts() {
        let mut model = Gna2Model::new();
        let op = Gna2Operation {
            op_type: OperationType::Copy,
            number_of_operands: 0,
            number_of_parameters: 0,
            operands: vec![],
            parameters: vec![],
        };
        model.add_operation(op.clone());
        let software_model = SoftwareModel::new(model);

        assert_eq!(software_model.operation_count(), 1);
        assert_eq!(software_model.get_operation(0), Some(&op));
        assert_eq!(software_model.get_operations().len(), 1);
    }

    #[test]
    fn software_model_maximum_operand_size_caches_value() {
        let mut model = Gna2Model::new();
        let op = Gna2Operation {
            op_type: OperationType::Copy,
            number_of_operands: 1,
            number_of_parameters: 0,
            operands: vec![Some(Default::default())],
            parameters: vec![],
        };
        model.add_operation(op);
        let mut software_model = SoftwareModel::new(model);

        assert_eq!(software_model.get_maximum_operand_size(0), 1);
        assert_eq!(software_model.get_maximum_operand_size(0), 1);
    }
}
