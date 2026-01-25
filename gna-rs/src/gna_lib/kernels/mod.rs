/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Kernel primitives and helpers (ported from `gna-lib` kernels and kernel-related headers).

pub mod kernel_arguments;
pub use kernel_arguments::KernelArguments;

pub mod xnn_kernel;
pub use xnn_kernel::{KernelMap, AccelerationMode, KernelFn};

pub mod acceleration_detector;
pub use acceleration_detector::AccelerationDetector;

pub mod transpose;
pub use transpose::{transpose_i16, transpose_i8};

// Stubs for original kernels (auto-generated)
pub mod common_hpp;
pub mod common_sse4_hpp;
pub mod common_avx1_hpp;
pub mod common_avx2_hpp;
pub mod cmakelists_txt;

pub mod affine_avx2_sat;
pub mod affine_sse4_sat;

pub mod convnet_avx1_sat;
pub mod convnet_avx2_sat;
pub mod convnet_generic_sat;
pub mod convnet_sse4_sat;
pub mod convnet_h;
pub mod convolution_kernel_arguments;

pub mod gmm;

pub mod igemm8_generic_sat;
pub mod igemm8_sse4_sat;
pub mod igemm8_avx2_sat;
pub mod igemm8_avx1_sat;
pub mod igemm8_subset_avx2_sat;
pub mod igemm8_subset_avx1_sat;
pub mod igemm8_subset_sse4_sat;

pub mod igemm16_subset_sse4_sat;
pub mod igemm16_subset_generic_sat;
pub mod igemm16_subset_avx2_sat;
pub mod igemm16_subset_avx1_sat;

pub mod igemm16_sse4_sat;
pub mod igemm16_generic_sat;
pub mod igemm16_avx2_sat;
pub mod igemm16_avx1_sat;

pub mod igemv16_avx1_sat;
pub mod igemv16_avx2_sat;
pub mod igemv16_sse4_sat;
pub mod igemv16_generic_sat;

pub mod igemv8_sse4_sat;
pub mod igemv8_generic_sat;
pub mod igemv8_avx2_sat;
pub mod igemv8_avx1_sat;
pub mod igemv8_h;

pub mod isbmm8;
pub mod isbmm16;

pub mod kernel_gmm;

pub mod transpose8_sse4_rs;
pub mod transpose8_generic_rs;
pub mod transpose8_avx2_rs;

pub mod transpose16_sse4_rs;
pub mod transpose16_generic_rs;
pub mod transpose16_avx2_rs;
pub mod transpose16_avx1_rs;

pub mod saturate_h;

pub mod rnn_sse4_sat;
pub mod rnn_avx2_sat;

pub mod pwl;
pub mod pooling_kernel_arguments;

pub mod kernel_macros;
