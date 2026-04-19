/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_lib::compiled_model::CompiledModel;
use crate::gna_rs::gna_lib::hardware_capabilities::HardwareCapabilities;
use crate::gna_rs::gna_lib::hardware_model::HardwareModel;

/// Simplified Rust port of the GNA `HardwareModelTarget` helper.
#[derive(Debug)]
pub struct HardwareModelTarget {
    hardware_model: HardwareModel,
}

impl HardwareModelTarget {
    pub fn new(software_model: &CompiledModel, hw_capabilities: HardwareCapabilities) -> Self {
        let mut model = Self {
            hardware_model: HardwareModel::new(software_model, hw_capabilities),
        };
        model.hardware_model.build();
        model
    }

    pub fn layer_count(&self) -> usize {
        self.hardware_model.layer_count()
    }

    pub fn get_layer(
        &self,
        layer_index: usize,
    ) -> &crate::gna_rs::gna_lib::hardware_layer::HardwareLayer {
        self.hardware_model.get_layer(layer_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn hardware_model_target_builds_hardware_model() {
        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());
        let compiled = CompiledModel::new(model);

        let target = HardwareModelTarget::new(&compiled, HardwareCapabilities);
        assert_eq!(target.layer_count(), 1);
        assert!(target.get_layer(0).get_ld_input_offset() >= 0);
    }
}
