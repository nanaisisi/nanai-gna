/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for Shape

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Shape(pub Vec<usize>);

impl Shape {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_dims(dimensions: Vec<usize>) -> Self {
        Self(dimensions)
    }

    pub fn at(&self, index: usize) -> usize {
        self.0.get(index).copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
