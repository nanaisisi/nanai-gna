/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for HardwareModelNoMMU

#[allow(dead_code)]
pub struct HardwareModelNoMMU;

impl HardwareModelNoMMU {
    pub fn supports_no_mmu() -> bool { true }
}
