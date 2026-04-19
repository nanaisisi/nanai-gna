/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::model_api::Gna2Model;
use crate::gna_rs::gna_lib::iscorable::IScorable;
use crate::gna_rs::gna_lib::software_model::SoftwareModel;

/// Stubbed Rust port of the original GNA `SoftwareOnlyModel` helper.
#[derive(Debug)]
pub struct SoftwareOnlyModel {
    pub software_model: SoftwareModel,
}

impl SoftwareOnlyModel {
    pub fn new(model: Gna2Model) -> Self {
        Self {
            software_model: SoftwareModel::new(model),
        }
    }

    pub fn default() -> Self {
        Self::new(Gna2Model::new())
    }

    pub fn is_software_only(&self) -> bool {
        true
    }

    pub fn invalidate_request_config(&mut self, _config_id: u32) {
        // No-op for this simplified port.
    }

    pub fn validate_buffer(&self, _request_allocations: &[u8], _memory: &[u8]) {
        // No-op for the simplified implementation.
    }

    pub fn is_fully_hardware_compatible(&self) -> bool {
        false
    }
}

impl IScorable for SoftwareOnlyModel {
    fn score(&self) -> u32 {
        self.software_model.score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn software_only_model_new_creates_instance() {
        let model = SoftwareOnlyModel::default();
        assert!(model.is_software_only());
    }

    #[test]
    fn software_only_model_scores_and_returns_zero() {
        let model = SoftwareOnlyModel::default();
        assert_eq!(model.score(), 0);
    }

    #[test]
    fn software_only_model_reports_not_hardware_compatible() {
        let model = SoftwareOnlyModel::default();
        assert!(!model.is_fully_hardware_compatible());
    }

    #[test]
    fn software_only_model_with_model_contains_operations() {
        let mut api_model = Gna2Model::new();
        api_model.add_operation(crate::gna_rs::gna_api::model_api::Gna2Operation {
            op_type: OperationType::Copy,
            number_of_operands: 0,
            number_of_parameters: 0,
            operands: vec![],
            parameters: vec![],
        });
        let model = SoftwareOnlyModel::new(api_model);

        assert_eq!(model.software_model.operation_count(), 1);
    }
}
