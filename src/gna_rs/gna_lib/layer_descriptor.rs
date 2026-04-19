/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::collections::HashMap;

use crate::gna_api::types::OperationType;

/// Simplified Rust port of the GNA `LayerDescriptor` helper.
#[derive(Debug, Clone)]
pub struct LayerDescriptor {
    layer_index: usize,
    operation: OperationType,
    input_count: usize,
    output_count: usize,
    parameters: HashMap<String, u32>,
}

impl LayerDescriptor {
    pub fn new(
        layer_index: usize,
        operation: OperationType,
        input_count: usize,
        output_count: usize,
    ) -> Self {
        Self {
            layer_index,
            operation,
            input_count,
            output_count,
            parameters: HashMap::new(),
        }
    }

    pub fn set_parameter(&mut self, key: impl Into<String>, value: u32) {
        self.parameters.insert(key.into(), value);
    }

    pub fn get_parameter(&self, key: &str) -> Option<u32> {
        self.parameters.get(key).copied()
    }

    pub fn layer_index(&self) -> usize {
        self.layer_index
    }

    pub fn operation(&self) -> OperationType {
        self.operation
    }

    pub fn input_count(&self) -> usize {
        self.input_count
    }

    pub fn output_count(&self) -> usize {
        self.output_count
    }

    pub fn describe(&self) -> String {
        let params: Vec<String> = self
            .parameters
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        format!(
            "LayerDescriptor(index={}, op={:?}, inputs={}, outputs={}, params=[{}])",
            self.layer_index,
            self.operation,
            self.input_count,
            self.output_count,
            params.join(", "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::LayerDescriptor;
    use crate::gna_api::types::OperationType;

    #[test]
    fn layer_descriptor_can_store_and_retrieve_parameters() {
        let mut desc = LayerDescriptor::new(0, OperationType::Copy, 1, 1);
        desc.set_parameter("weight_size", 16);
        assert_eq!(desc.get_parameter("weight_size"), Some(16));
        assert_eq!(desc.layer_index(), 0);
        assert_eq!(desc.operation(), OperationType::Copy);
        assert_eq!(desc.input_count(), 1);
        assert_eq!(desc.output_count(), 1);
    }

    #[test]
    fn layer_descriptor_describe_includes_parameter_list() {
        let mut desc = LayerDescriptor::new(1, OperationType::Convolution, 2, 1);
        desc.set_parameter("kernel_size", 3);
        let description = desc.describe();
        assert!(description.contains("index=1"));
        assert!(description.contains("op=Convolution"));
        assert!(description.contains("inputs=2"));
        assert!(description.contains("outputs=1"));
        assert!(description.contains("kernel_size=3"));
    }
}
