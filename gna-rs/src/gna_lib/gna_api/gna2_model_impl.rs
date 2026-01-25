#![allow(dead_code)]

/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/gna-api/gna2-model-impl.h / .cpp


/// Model management stubs

pub fn model_create(_device_index: u32) -> Result<u32, ()> { Ok(0) }
pub fn model_release(_model_id: u32) -> bool { true }
pub fn model_get_last_error() -> Option<String> { None }
