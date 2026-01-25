/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `Tensor` type.

#[derive(Debug, Default, Clone)]
pub struct Tensor {
    pub size: usize,
}

impl Tensor {
    pub fn new(size: usize) -> Self { Self { size } }
}
