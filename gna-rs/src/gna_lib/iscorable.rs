/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for IScorable trait

#[allow(dead_code)]
pub trait IScorable {
    fn score(&self) -> u32;
}
