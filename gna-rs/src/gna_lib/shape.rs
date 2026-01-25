/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for Shape

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Shape(pub Vec<usize>);

impl Shape {
    pub fn new() -> Self { Self(Vec::new()) }
}
