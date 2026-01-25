#![allow(dead_code)]

/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/gna-api/gna2-instrumentation-impl.h / .cpp


/// Instrumentation API stubs

pub fn instrumentation_config_create(_num_points: u32) -> u32 { 0 }
pub fn instrumentation_config_set_mode(_id: u32, _mode: u32) -> bool { true }
pub fn instrumentation_config_set_unit(_id: u32, _unit: u32) -> bool { true }
pub fn instrumentation_config_release(_id: u32) -> bool { true }
