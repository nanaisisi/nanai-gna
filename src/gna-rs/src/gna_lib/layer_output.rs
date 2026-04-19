/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Rust port of the GNA `LayerOutput` helper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerOutput {
    buffer: Vec<u8>,
    scratchpad: Vec<u8>,
    grouping: u32,
    element_count: u32,
}

impl LayerOutput {
    pub fn new(grouping: u32, element_count: u32) -> Self {
        Self {
            buffer: Vec::new(),
            scratchpad: Vec::new(),
            grouping,
            element_count,
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.clear();
        self.buffer.extend_from_slice(data);
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer
    }

    pub fn scratchpad_mut(&mut self) -> &mut Vec<u8> {
        &mut self.scratchpad
    }

    pub fn scratchpad(&self) -> &[u8] {
        &self.scratchpad
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
    use super::LayerOutput;

    #[test]
    fn layer_output_writes_and_reads_data() {
        let mut output = LayerOutput::new(2, 4);
        output.write(&[9, 8, 7]);
        assert_eq!(output.data(), &[9, 8, 7]);
        assert_eq!(output.grouping(), 2);
        assert_eq!(output.element_count(), 4);
    }

    #[test]
    fn layer_output_scratchpad_is_available_for_writes() {
        let mut output = LayerOutput::new(1, 1);
        output.scratchpad_mut().extend_from_slice(&[0, 1, 2]);
        assert_eq!(output.scratchpad(), &[0, 1, 2]);
    }
}
