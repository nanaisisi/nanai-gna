/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::types::OperationType;
use crate::gna_lib::layer_descriptor::LayerDescriptor;

/// Simplified Rust port of the GNA `HardwareLayer` helper.
#[derive(Debug, Clone)]
pub struct HardwareLayer {
    operation: OperationType,
    descriptor: LayerDescriptor,
    activation_disabled: bool,
    configured: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NnopType {
    Affine = 0,
    AffineActiveList = 1,
    Gmm = 2,
    GmmActiveList = 3,
    Cnn = 4,
    Rnn = 5,
    Copy = 6,
    Transposition = 7,
    Unknown = 255,
}

impl NnopType {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

impl HardwareLayer {
    const OUT_BUFFER: &'static str = "out_buffer";
    const OUT_SUM_BUFFER: &'static str = "out_sum_buffer";
    const IN_BUFFER: &'static str = "in_buffer";
    const WEIGHT_BUFFER: &'static str = "weight_buffer";
    const BIAS_BUFFER: &'static str = "bias_buffer";
    const PWL_SEG_DEF_BUFFER: &'static str = "pwl_seg_def_buffer";
    const ACT_LIST_BUFFER: &'static str = "act_list_buffer";
    const ACT_LIST_N_ELEMS: &'static str = "act_list_n_elems";
    const GMM_DESCRIPTOR: &'static str = "gmm_descriptor";
    const GMM_SCRLEN: &'static str = "gmmscrlen";
    const INTERMEDIATE_OUTPUT_BUFFER: &'static str = "intermediate_output_buffer";
    const FEEDBACK_BUFFER: &'static str = "feedback_buffer";
    const OP: &'static str = "op";
    const XNN_DESCRIPTOR_OFFSET: &'static str = "xnn_descriptor_offset";

    pub fn new(
        operation: OperationType,
        descriptor: LayerDescriptor,
        activation_disabled: bool,
    ) -> Self {
        Self {
            operation,
            descriptor,
            activation_disabled,
            configured: false,
        }
    }

    pub fn configure(&mut self) -> bool {
        self.configured = true;
        true
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }
}

impl Default for HardwareLayer {
    fn default() -> Self {
        Self::new(
            OperationType::Copy,
            LayerDescriptor::new(0, OperationType::Copy, 0, 0),
            false,
        )
    }
}

impl HardwareLayer {
    pub fn operation(&self) -> OperationType {
        self.operation
    }

    pub fn activation_disabled(&self) -> bool {
        self.activation_disabled
    }

    fn descriptor_offset(&self, key: &str) -> u32 {
        self.descriptor.get_parameter(key).unwrap_or(0)
    }

    pub fn get_xnn_descriptor_offset(&self) -> u32 {
        self.descriptor_offset(Self::XNN_DESCRIPTOR_OFFSET)
    }

    pub fn get_gmm_descriptor_offset(&self) -> u32 {
        self.descriptor_offset(Self::GMM_DESCRIPTOR)
    }

    pub fn get_ld_nnop_offset(&self) -> u32 {
        self.descriptor_offset(Self::OP)
    }

    pub fn get_ld_input_offset(&self) -> u32 {
        self.descriptor_offset(Self::IN_BUFFER)
    }

    pub fn get_ld_output_offset(&self) -> u32 {
        if !self.activation_disabled
            || matches!(
                self.operation,
                OperationType::Convolution
                    | OperationType::Transposition
                    | OperationType::Copy
                    | OperationType::Recurrent
            )
        {
            self.descriptor_offset(Self::OUT_BUFFER)
        } else {
            self.descriptor_offset(Self::OUT_SUM_BUFFER)
        }
    }

    pub fn get_ld_weight_offset(&self) -> u32 {
        self.descriptor_offset(Self::WEIGHT_BUFFER)
    }

    pub fn get_ld_bias_offset(&self) -> u32 {
        self.descriptor_offset(Self::BIAS_BUFFER)
    }

    pub fn get_ld_pwl_offset(&self) -> u32 {
        self.descriptor_offset(Self::PWL_SEG_DEF_BUFFER)
    }

    pub fn get_ld_actlist_offset(&self) -> u32 {
        self.descriptor_offset(Self::ACT_LIST_BUFFER)
    }

    pub fn get_ld_actlen_offset(&self) -> u32 {
        self.descriptor_offset(Self::ACT_LIST_N_ELEMS)
    }

    pub fn get_ld_scrlen_offset(&self) -> u32 {
        self.descriptor_offset(Self::GMM_SCRLEN)
    }

    pub fn get_ld_intermediate_output_offset(&self) -> u32 {
        self.descriptor_offset(Self::INTERMEDIATE_OUTPUT_BUFFER)
    }

    pub fn get_ld_feedback_offset(&self) -> u32 {
        self.descriptor_offset(Self::FEEDBACK_BUFFER)
    }

    pub fn descriptor(&self) -> &LayerDescriptor {
        &self.descriptor
    }

    pub fn get_nn_op_type(&self, has_active_list: bool) -> NnopType {
        match self.operation {
            OperationType::FullyConnectedAffine | OperationType::ElementWiseAffine => {
                if has_active_list {
                    NnopType::AffineActiveList
                } else {
                    NnopType::Affine
                }
            }
            OperationType::Gmm => {
                if has_active_list {
                    NnopType::GmmActiveList
                } else {
                    NnopType::Gmm
                }
            }
            OperationType::Convolution => NnopType::Cnn,
            OperationType::Recurrent => NnopType::Rnn,
            OperationType::Copy => NnopType::Copy,
            OperationType::Transposition => NnopType::Transposition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HardwareLayer, NnopType};
    use crate::gna_api::types::OperationType;
    use crate::gna_lib::layer_descriptor::LayerDescriptor;

    #[test]
    fn hardware_layer_offsets_come_from_descriptor_parameters() {
        let mut descriptor = LayerDescriptor::new(0, OperationType::Copy, 1, 1);
        descriptor.set_parameter("xnn_descriptor_offset", 0x100);
        descriptor.set_parameter("in_buffer", 0x110);
        descriptor.set_parameter("out_buffer", 0x120);
        descriptor.set_parameter("weight_buffer", 0x130);
        descriptor.set_parameter("bias_buffer", 0x140);
        descriptor.set_parameter("pwl_seg_def_buffer", 0x150);
        descriptor.set_parameter("act_list_buffer", 0x160);
        descriptor.set_parameter("act_list_n_elems", 0x170);
        descriptor.set_parameter("gmm_descriptor", 0x180);
        descriptor.set_parameter("gmmscrlen", 0x190);

        let layer = HardwareLayer::new(OperationType::Copy, descriptor, false);

        assert_eq!(layer.get_xnn_descriptor_offset(), 0x100);
        assert_eq!(layer.get_ld_input_offset(), 0x110);
        assert_eq!(layer.get_ld_output_offset(), 0x120);
        assert_eq!(layer.get_ld_weight_offset(), 0x130);
        assert_eq!(layer.get_ld_bias_offset(), 0x140);
        assert_eq!(layer.get_ld_pwl_offset(), 0x150);
        assert_eq!(layer.get_ld_actlist_offset(), 0x160);
        assert_eq!(layer.get_ld_actlen_offset(), 0x170);
        assert_eq!(layer.get_gmm_descriptor_offset(), 0x180);
        assert_eq!(layer.get_ld_scrlen_offset(), 0x190);
    }

    #[test]
    fn hardware_layer_get_nn_op_type_maps_active_list_correctly() {
        let descriptor = LayerDescriptor::new(0, OperationType::FullyConnectedAffine, 1, 1);
        let layer = HardwareLayer::new(OperationType::FullyConnectedAffine, descriptor, true);

        assert_eq!(layer.get_nn_op_type(false), NnopType::Affine);
        assert_eq!(layer.get_nn_op_type(true), NnopType::AffineActiveList);
    }

    #[test]
    fn hardware_layer_output_offset_uses_out_sum_when_activation_disabled() {
        let mut descriptor = LayerDescriptor::new(0, OperationType::ElementWiseAffine, 1, 1);
        descriptor.set_parameter("out_buffer", 0x100);
        descriptor.set_parameter("out_sum_buffer", 0x200);

        let layer = HardwareLayer::new(OperationType::ElementWiseAffine, descriptor, true);
        assert_eq!(layer.get_ld_output_offset(), 0x200);
    }
}
