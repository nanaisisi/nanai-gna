/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for LayerOutput

#[allow(dead_code)]
pub struct LayerOutput {
    buffer: Vec<u8>,
}

impl LayerOutput {
    pub fn new() -> Self { Self { buffer: Vec::new() } }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.clear();
        self.buffer.extend_from_slice(data);
    }

    pub fn data(&self) -> &[u8] { &self.buffer }
}
