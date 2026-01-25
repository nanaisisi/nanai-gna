#![allow(dead_code)]

/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/gna-api/gna2-inference-impl.h / .cpp


/// Inference API stubs (request config, enqueue, wait)

pub fn request_config_create(_model_id: u32) -> u32 { 0 }

pub fn request_config_set_operand_buffer(_request_config_id: u32, _op_index: u32, _operand_index: u32, _addr: usize) -> bool { true }

pub fn request_enqueue(_request_config_id: u32) -> u32 { 0 }

pub fn request_wait(_request_id: u32, _timeout_ms: u32) -> bool { true }
