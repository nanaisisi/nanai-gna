/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Skeleton for `LayerConfiguration`.

use crate::gna_lib::BufferMap;

#[derive(Debug, Default)]
pub struct LayerConfiguration {
    pub buffers: BufferMap,
    // other config: activation lists, config lists
}

impl LayerConfiguration {
    pub fn new() -> Self { Self::default() }
}
