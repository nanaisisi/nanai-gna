/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `Layer` and related types.

use crate::gna_lib::buffer_map::BufferMap;

#[derive(Debug)]
pub struct Layer {
    // validator, transforms, input/output tensors
    pub buffers: BufferMap,
}

impl Layer {
    pub fn new() -> Self { Self { buffers: BufferMap::new() } }
}
