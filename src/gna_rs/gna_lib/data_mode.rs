/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode};

/// Simplified Rust port of the GNA `DataMode` helper.
///
/// This stores an element type, tensor mode, and element size in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataMode {
    pub data_type: Gna2DataType,
    pub mode: Gna2TensorMode,
    pub size: usize,
}

impl DataMode {
    pub fn new(data_type: Gna2DataType) -> Self {
        let mode = Self::mode_from_type(data_type);
        let size = Self::size_for_type(data_type);
        Self {
            data_type,
            mode,
            size,
        }
    }

    pub fn with_mode(data_type: Gna2DataType, mode: Gna2TensorMode) -> Self {
        let data_type = Self::type_from_mode(data_type, mode);
        let size = Self::size_for_type(data_type);
        Self {
            data_type,
            mode,
            size,
        }
    }

    pub fn set_mode(&mut self, mode: Gna2TensorMode) {
        self.data_type = Self::type_from_mode(self.data_type, mode);
        self.mode = mode;
        self.size = Self::size_for_type(self.data_type);
    }

    pub fn size_for_type(data_type: Gna2DataType) -> usize {
        match data_type {
            Gna2DataType::None => 0,
            Gna2DataType::Boolean => 1,
            Gna2DataType::Int4 => 1,
            Gna2DataType::Int8 => 1,
            Gna2DataType::Int16 => 2,
            Gna2DataType::Int32 => 4,
            Gna2DataType::Int64 => 8,
            Gna2DataType::Uint4 => 1,
            Gna2DataType::Uint8 => 1,
            Gna2DataType::Uint16 => 2,
            Gna2DataType::Uint32 => 4,
            Gna2DataType::Uint64 => 8,
            Gna2DataType::CompoundBias => 8,
            Gna2DataType::PwlSegment => 8,
            Gna2DataType::WeightScaleFactor => 8,
        }
    }

    pub fn mode_from_type(data_type: Gna2DataType) -> Gna2TensorMode {
        match data_type {
            Gna2DataType::None => Gna2TensorMode::Disabled,
            _ => Gna2TensorMode::Default,
        }
    }

    pub fn type_from_mode(data_type: Gna2DataType, mode: Gna2TensorMode) -> Gna2DataType {
        match mode {
            Gna2TensorMode::Disabled => Gna2DataType::None,
            Gna2TensorMode::ConstantScalar => Gna2DataType::Int4,
            _ => data_type,
        }
    }
}

impl Default for DataMode {
    fn default() -> Self {
        Self::new(Gna2DataType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::DataMode;
    use crate::gna_rs::gna_api::types::{Gna2DataType, Gna2TensorMode};

    #[test]
    fn data_mode_default_is_disabled_none() {
        let mode = DataMode::default();
        assert_eq!(mode.data_type, Gna2DataType::None);
        assert_eq!(mode.mode, Gna2TensorMode::Disabled);
        assert_eq!(mode.size, 0);
    }

    #[test]
    fn data_mode_new_sets_default_mode_and_size() {
        let mode = DataMode::new(Gna2DataType::Int16);
        assert_eq!(mode.data_type, Gna2DataType::Int16);
        assert_eq!(mode.mode, Gna2TensorMode::Default);
        assert_eq!(mode.size, 2);
    }

    #[test]
    fn set_mode_updates_data_type_for_disabled() {
        let mut mode = DataMode::new(Gna2DataType::Int32);
        mode.set_mode(Gna2TensorMode::Disabled);
        assert_eq!(mode.data_type, Gna2DataType::None);
        assert_eq!(mode.mode, Gna2TensorMode::Disabled);
        assert_eq!(mode.size, 0);
    }

    #[test]
    fn with_mode_uses_external_buffer_without_changing_data_type() {
        let mode = DataMode::with_mode(Gna2DataType::Int8, Gna2TensorMode::ExternalBuffer);
        assert_eq!(mode.data_type, Gna2DataType::Int8);
        assert_eq!(mode.mode, Gna2TensorMode::ExternalBuffer);
        assert_eq!(mode.size, 1);
    }

    #[test]
    fn data_mode_handles_boolean_and_int4_sizes() {
        let boolean_mode = DataMode::new(Gna2DataType::Boolean);
        assert_eq!(boolean_mode.data_type, Gna2DataType::Boolean);
        assert_eq!(boolean_mode.mode, Gna2TensorMode::Default);
        assert_eq!(boolean_mode.size, 1);

        let int4_mode = DataMode::new(Gna2DataType::Int4);
        assert_eq!(int4_mode.data_type, Gna2DataType::Int4);
        assert_eq!(int4_mode.mode, Gna2TensorMode::Default);
        assert_eq!(int4_mode.size, 1);
    }

    #[test]
    fn set_mode_constant_scalar_changes_type_to_int4() {
        let mut mode = DataMode::new(Gna2DataType::Int16);
        mode.set_mode(Gna2TensorMode::ConstantScalar);
        assert_eq!(mode.data_type, Gna2DataType::Int4);
        assert_eq!(mode.mode, Gna2TensorMode::ConstantScalar);
        assert_eq!(mode.size, 1);
    }

    #[test]
    fn with_mode_constant_scalar_uses_int4_type() {
        let mode = DataMode::with_mode(Gna2DataType::Int32, Gna2TensorMode::ConstantScalar);
        assert_eq!(mode.data_type, Gna2DataType::Int4);
        assert_eq!(mode.mode, Gna2TensorMode::ConstantScalar);
        assert_eq!(mode.size, 1);
    }
}
