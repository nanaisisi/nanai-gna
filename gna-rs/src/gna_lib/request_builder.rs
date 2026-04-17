/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::collections::BTreeMap;

use crate::gna_lib::{Request, RequestConfiguration};

/// Simplified Rust port of the GNA `RequestBuilder` helper.
#[derive(Debug, Default)]
pub struct RequestBuilder {
    configurations: BTreeMap<u32, RequestConfiguration>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            configurations: BTreeMap::new(),
        }
    }

    pub fn create_configuration(&mut self, config: RequestConfiguration) -> u32 {
        let config_id = config.config_id;
        self.configurations.insert(config_id, config);
        config_id
    }

    pub fn release_configuration(&mut self, config_id: u32) -> bool {
        self.configurations.remove(&config_id).is_some()
    }

    pub fn attach_buffer(
        &mut self,
        config_id: u32,
        operand_index: u32,
        address: crate::common::BaseAddress,
    ) -> bool {
        if let Some(config) = self.configurations.get_mut(&config_id) {
            config.set_buffer(operand_index, address);
            true
        } else {
            false
        }
    }

    pub fn get_configuration(&self, config_id: u32) -> Option<&RequestConfiguration> {
        self.configurations.get(&config_id)
    }

    pub fn get_configuration_mut(&mut self, config_id: u32) -> Option<&mut RequestConfiguration> {
        self.configurations.get_mut(&config_id)
    }

    pub fn create_request(&self, config_id: u32) -> Option<Request> {
        self.configurations
            .get(&config_id)
            .cloned()
            .map(Request::new)
    }

    pub fn has_configuration(&self, config_id: u32) -> bool {
        self.configurations.contains_key(&config_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{RequestBuilder, RequestConfiguration};
    use crate::common::BaseAddress;

    #[test]
    fn request_builder_manages_configuration_lifecycle() {
        let mut builder = RequestBuilder::new();
        let config = RequestConfiguration::new();
        let config_id = config.config_id;

        assert!(!builder.has_configuration(config_id));

        builder.create_configuration(config);
        assert!(builder.has_configuration(config_id));

        let request = builder.create_request(config_id);
        assert!(request.is_some());
        assert_eq!(request.unwrap().config.config_id, config_id);

        assert!(builder.release_configuration(config_id));
        assert!(!builder.has_configuration(config_id));
    }

    #[test]
    fn request_builder_attach_buffer_updates_configuration() {
        let mut builder = RequestBuilder::new();
        let config = RequestConfiguration::new();
        let config_id = config.config_id;
        builder.create_configuration(config);

        let address = BaseAddress::from(0 as *mut u8);
        assert!(builder.attach_buffer(config_id, 1, address));
        let saved = builder.get_configuration(config_id).unwrap();
        assert_eq!(saved.get_buffer(1), Some(address));
    }
}
