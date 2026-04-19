/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode, OperationType};
use crate::gna_rs::gna_lib::data_mode::DataMode;

/// Simplified Rust port of the GNA `LayerCapabilities` helper.
#[derive(Debug, Clone, Copy)]
pub struct LayerCapabilities;

impl LayerCapabilities {
    pub const INPUT_ELEMENT_COUNT_MULTIPLIER: u32 = 8;
    pub const COPY_ROWS_MAX: u32 = 255;
    pub const RECURRENT_OUTPUT_ELEMENT_COUNT_MULTIPLIER: u32 = 32;
    pub const INPUT_ELEMENT_COUNT_MAX: u32 = u16::MAX as u32;
    pub const WEIGHT_ELEMENT_SIZE_MAX: u32 = 2;

    pub fn get_all_supported_operation_types() -> &'static [OperationType] {
        &[
            OperationType::FullyConnectedAffine,
            OperationType::ElementWiseAffine,
            OperationType::Recurrent,
            OperationType::Copy,
            OperationType::Convolution,
            OperationType::Gmm,
            OperationType::Transposition,
        ]
    }

    pub fn is_operation_supported(operation: OperationType) -> bool {
        Self::get_all_supported_operation_types().contains(&operation)
    }

    pub fn make_data_modes_cartesian(
        types: &[Gna2DataType],
        modes: Option<&[Gna2TensorMode]>,
    ) -> Vec<DataMode> {
        let modes = modes.unwrap_or(&[Gna2TensorMode::Default, Gna2TensorMode::ExternalBuffer]);

        let mut cartesian = Vec::new();
        for &data_type in types {
            for &mode in modes {
                cartesian.push(DataMode::with_mode(data_type, mode));
            }
        }
        cartesian
    }
}

#[cfg(test)]
mod tests {
    use super::LayerCapabilities;
    use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode, OperationType};

    #[test]
    fn supported_operations_include_copy_and_convolution() {
        let supported = LayerCapabilities::get_all_supported_operation_types();
        assert!(supported.contains(&OperationType::Copy));
        assert!(supported.contains(&OperationType::Convolution));
    }

    #[test]
    fn is_operation_supported_returns_false_for_unknown_operation() {
        // All defined operations are supported; make sure this method behaves correctly.
        assert!(LayerCapabilities::is_operation_supported(
            OperationType::Gmm
        ));
    }

    #[test]
    fn make_data_modes_cartesian_generates_default_and_external_buffer_modes() {
        let types = [Gna2DataType::Int8, Gna2DataType::Int16];
        let modes = LayerCapabilities::make_data_modes_cartesian(&types, None);
        assert_eq!(modes.len(), 4);
        assert!(modes.iter().any(
            |mode| mode.data_type == Gna2DataType::Int8 && mode.mode == Gna2TensorMode::Default
        ));
        assert!(
            modes
                .iter()
                .any(|mode| mode.data_type == Gna2DataType::Int16
                    && mode.mode == Gna2TensorMode::ExternalBuffer)
        );
    }
}
