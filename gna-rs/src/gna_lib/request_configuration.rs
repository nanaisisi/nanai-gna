/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `RequestConfiguration`.
use crate::common::BaseAddress;
use crate::gna_lib::active_list::ActiveList;
use crate::gna_lib::layer_configuration::LayerConfiguration;
use crate::gna_lib::profiler_configuration::ProfilerConfiguration;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone)]
pub struct RequestConfiguration {
    pub buffers: BTreeMap<u32, BaseAddress>,
    pub layer_configurations: BTreeMap<u32, LayerConfiguration>,
    pub timeout_ms: u32,
    pub config_id: u32,
    pub active_list_count: u32,
    pub acceleration_mode: crate::gna_api::inference_api::Gna2AccelerationMode,
    /// Optional instrumentation points to collect for this request
    pub instrumentation_points: Vec<crate::gna_api::instrumentation_api::Gna2InstrumentationPoint>,
    pub profiler_configuration: Option<ProfilerConfiguration>,
}

static NEXT_CONFIG_ID: AtomicU32 = AtomicU32::new(1);

impl Default for RequestConfiguration {
    fn default() -> Self {
        Self {
            buffers: BTreeMap::new(),
            layer_configurations: BTreeMap::new(),
            timeout_ms: 1000,
            config_id: NEXT_CONFIG_ID.fetch_add(1, Ordering::Relaxed),
            active_list_count: 0,
            acceleration_mode: crate::gna_api::inference_api::Gna2AccelerationMode::default(),
            instrumentation_points: Vec::new(),
            profiler_configuration: None,
        }
    }
}

impl RequestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_buffer(&mut self, operand_index: u32, addr: BaseAddress) {
        self.buffers.insert(operand_index, addr);
    }

    pub fn add_buffer(&mut self, layer_index: u32, operand_index: u32, addr: BaseAddress) {
        self.buffers.insert(operand_index, addr);
        self.layer_configurations
            .entry(layer_index)
            .or_default()
            .set_buffer(operand_index, addr);
    }

    pub fn get_buffer(&self, operand_index: u32) -> Option<BaseAddress> {
        self.buffers.get(&operand_index).cloned()
    }

    pub fn get_layer_configuration(&self, layer_index: u32) -> Option<&LayerConfiguration> {
        self.layer_configurations.get(&layer_index)
    }

    pub fn get_layer_configuration_mut(
        &mut self,
        layer_index: u32,
    ) -> Option<&mut LayerConfiguration> {
        self.layer_configurations.get_mut(&layer_index)
    }

    pub fn set_instrumentation_points(
        &mut self,
        pts: &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint],
    ) {
        self.instrumentation_points = pts.to_vec();
    }

    pub fn get_instrumentation_points(
        &self,
    ) -> &[crate::gna_api::instrumentation_api::Gna2InstrumentationPoint] {
        &self.instrumentation_points
    }

    pub fn set_acceleration_mode(
        &mut self,
        mode: crate::gna_api::inference_api::Gna2AccelerationMode,
    ) {
        self.acceleration_mode = mode;
    }

    pub fn get_acceleration_mode(&self) -> crate::gna_api::inference_api::Gna2AccelerationMode {
        self.acceleration_mode
    }

    pub fn add_active_list(&mut self, layer_index: u32, active_list: ActiveList) -> bool {
        let config = self.layer_configurations.entry(layer_index).or_default();
        let result = config.set_active_list(active_list);
        if result {
            self.active_list_count += 1;
        }
        result
    }

    pub fn assign_profiler_config(&mut self, config: ProfilerConfiguration) {
        self.profiler_configuration = Some(config);
    }

    pub fn get_hw_instrumentation_mode(&self) -> u8 {
        self.profiler_configuration
            .as_ref()
            .map(|cfg| if cfg.is_enabled() { 1 } else { 0 })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BaseAddress;

    #[test]
    fn request_configuration_adds_layer_buffer() {
        let mut config = RequestConfiguration::new();
        let addr = BaseAddress::from_ptr(0x5000usize as *mut u8);
        config.add_buffer(1, 2, addr);

        assert_eq!(config.get_buffer(2), Some(addr));
        let layer_config = config.get_layer_configuration(1).unwrap();
        assert_eq!(layer_config.get_buffer(2), Some(addr));
    }

    #[test]
    fn request_configuration_attaches_active_list() {
        let mut config = RequestConfiguration::new();
        let mut active_list = ActiveList::new();
        active_list.add(4);

        assert!(config.add_active_list(0, active_list.clone()));
        assert_eq!(config.active_list_count, 1);
        let layer_config = config.get_layer_configuration(0).unwrap();
        assert!(layer_config.has_active_list());
        assert_eq!(layer_config.get_active_list(), Some(&active_list));
    }

    #[test]
    fn request_configuration_assigns_profiler_configuration() {
        let mut config = RequestConfiguration::new();
        let mut profiler = ProfilerConfiguration::new();
        profiler.enable(true);

        config.assign_profiler_config(profiler);
        assert_eq!(config.get_hw_instrumentation_mode(), 1);
    }
}
