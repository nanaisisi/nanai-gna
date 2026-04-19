/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::common::address::BaseAddress;
use crate::gna_rs::gna_api::types::{Gna2BiasMode, Gna2TensorMode};
use crate::gna_rs::gna_lib::data_mode::DataMode;

/// Minimal port of GNA bias tensor handling.
#[derive(Debug, Clone)]
pub struct Bias {
    pub dimensions: Vec<u32>,
    pub vector_index: u32,
    pub data_mode: DataMode,
    pub buffer: Vec<u8>,
    pub bias_mode: Gna2BiasMode,
}

impl Bias {
    pub fn new(
        dimensions: Vec<u32>,
        vector_index: u32,
        data_mode: DataMode,
        buffer: Vec<u8>,
        bias_mode: Gna2BiasMode,
    ) -> Self {
        Self {
            dimensions,
            vector_index,
            data_mode,
            buffer,
            bias_mode,
        }
    }

    pub fn apply(&self, target: &mut [u8]) {
        let copy_len = target.len().min(self.buffer.len());
        target[..copy_len].copy_from_slice(&self.buffer[..copy_len]);
    }

    pub fn base_address(&self) -> BaseAddress {
        BaseAddress::from_ptr(self.buffer.as_ptr() as *mut u8)
    }

    pub fn is_constant_scalar(&self) -> bool {
        self.data_mode.mode == Gna2TensorMode::ConstantScalar
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gna_rs::gna_api::types::Gna2DataType;

    #[test]
    fn bias_apply_copies_buffer_to_target() {
        let bias = Bias::new(
            vec![4],
            0,
            DataMode::new(Gna2DataType::Int8),
            vec![1, 2, 3, 4],
            Gna2BiasMode::Default,
        );
        let mut target = [0u8; 4];
        bias.apply(&mut target);
        assert_eq!(target, [1, 2, 3, 4]);
    }

    #[test]
    fn bias_base_address_is_not_null_for_buffer() {
        let bias = Bias::new(
            vec![2],
            0,
            DataMode::new(Gna2DataType::Int16),
            vec![0, 0],
            Gna2BiasMode::Default,
        );
        assert!(!bias.base_address().is_null());
    }

    #[test]
    fn bias_constant_scalar_detects_mode() {
        let mut data_mode = DataMode::new(Gna2DataType::Int4);
        data_mode.set_mode(Gna2TensorMode::ConstantScalar);
        let bias = Bias::new(vec![1], 0, data_mode, vec![0], Gna2BiasMode::Default);
        assert!(bias.is_constant_scalar());
    }
}
