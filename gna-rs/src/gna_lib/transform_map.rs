/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for TransformMap and transform registration.

use crate::gna_lib::transform::BaseTransform;

#[derive(Debug, Default)]
pub struct TransformMap {
    list: Vec<BaseTransform>,
}

impl TransformMap {
    pub fn new() -> Self { Self::default() }
}
