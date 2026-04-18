/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Rust port of the GNA `LayerInput` helper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerInput {
    buffer: Vec<u8>,
    grouping: u32,
    element_count: u32,
}

impl LayerInput {
    pub fn new(buffer: Vec<u8>, grouping: u32, element_count: u32) -> Self {
        Self {
            buffer,
            grouping,
            element_count,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            buffer: bytes.to_vec(),
            grouping: 1,
            element_count: bytes.len() as u32,
        }
    }

    pub fn read(&self) -> &[u8] {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn grouping(&self) -> u32 {
        self.grouping
    }

    pub fn element_count(&self) -> u32 {
        self.element_count
    }
}

#[cfg(test)]
mod tests {
    use super::LayerInput;

    #[test]
    fn layer_input_stores_and_reads_buffer() {
        let data = [1u8, 2, 3, 4];
        let input = LayerInput::from_bytes(&data);
        assert_eq!(input.read(), &data);
        assert_eq!(input.len(), 4);
        assert_eq!(input.grouping(), 1);
        assert_eq!(input.element_count(), 4);
    }

    #[test]
    fn layer_input_preserves_metadata() {
        let buffer = vec![0u8, 1, 2];
        let input = LayerInput::new(buffer.clone(), 4, 8);
        assert_eq!(input.read(), buffer.as_slice());
        assert_eq!(input.grouping(), 4);
        assert_eq!(input.element_count(), 8);
    }
}
