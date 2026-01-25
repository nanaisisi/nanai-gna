#![allow(dead_code)]

/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/gna-api/gna2-model-export-impl.h / .cpp


/// Model export stubs

pub fn model_export_config_create() -> u32 { 0 }
pub fn model_export_config_set_source(_config_id: u32, _device_index: u32, _model_id: u32) -> bool { true }
pub fn model_export(_config_id: u32) -> Option<Vec<u8>> { None }
