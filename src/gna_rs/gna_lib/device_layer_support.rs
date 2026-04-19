/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_rs::gna_api::model_api::Gna2Operation;
use crate::gna_rs::gna_api::types::OperationType;
use crate::gna_rs::gna_lib::data_mode::DataMode;

/// Simplified Rust port of GNA `DeviceLayerSupport`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataConfig {
    pub input: DataMode,
    pub weight: DataMode,
    pub bias: DataMode,
    pub output: DataMode,
    pub is_activation_disabled: bool,
}

impl DataConfig {
    pub fn new(
        input: DataMode,
        weight: DataMode,
        bias: DataMode,
        output: DataMode,
        is_activation_disabled: bool,
    ) -> Self {
        Self {
            input,
            weight,
            bias,
            output,
            is_activation_disabled,
        }
    }
}

#[derive(Debug)]
pub struct DeviceLayerSupport;

impl DeviceLayerSupport {
    pub fn supports(
        &self,
        operation: OperationType,
        config: &DataConfig,
        device_version: Gna2DeviceVersion,
    ) -> bool {
        if device_version.0 == 0 {
            return false;
        }

        // Disallow disabled output or disabled inputs for meaningful operations.
        if config.input.mode == crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
            || config.output.mode == crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
        {
            return false;
        }

        match operation {
            OperationType::Copy => true,
            OperationType::FullyConnectedAffine | OperationType::ElementWiseAffine => {
                config.weight.mode != crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
                    && config.bias.mode != crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
            }
            OperationType::Convolution => {
                config.weight.mode != crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
            }
            OperationType::Gmm => {
                config.weight.mode != crate::gna_rs::gna_api::types::Gna2TensorMode::Disabled
            }
            OperationType::Recurrent => true,
            OperationType::Transposition => true,
        }
    }

    pub fn is_supported(
        &self,
        operation: &Gna2Operation,
        config: &DataConfig,
        device_version: Gna2DeviceVersion,
    ) -> bool {
        self.supports(operation.op_type, config, device_version)
    }
}

#[cfg(test)]
mod tests {
    use super::{DataConfig, DeviceLayerSupport};
    use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
    use crate::gna_rs::gna_api::model_api::Gna2Operation;
    use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode};
    use crate::gna_rs::gna_lib::data_mode::DataMode;

    #[test]
    fn device_layer_support_allows_copy_with_valid_config() {
        let support = DeviceLayerSupport;
        let config = DataConfig::new(
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            false,
        );
        let op = Gna2Operation::default();
        assert!(support.is_supported(&op, &config, Gna2DeviceVersion(0x30)));
    }

    #[test]
    fn device_layer_support_rejects_disabled_output() {
        let support = DeviceLayerSupport;
        let config = DataConfig::new(
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::with_mode(Gna2DataType::Int16, Gna2TensorMode::Disabled),
            false,
        );
        let op = Gna2Operation::default();
        assert!(!support.is_supported(&op, &config, Gna2DeviceVersion(0x30)));
    }

    #[test]
    fn device_layer_support_rejects_invalid_version() {
        let support = DeviceLayerSupport;
        let config = DataConfig::new(
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            DataMode::new(Gna2DataType::Int16),
            false,
        );
        let op = Gna2Operation::default();
        assert!(!support.is_supported(&op, &config, Gna2DeviceVersion(0)));
    }
}
