//! Skeleton for `LayerConfiguration`.

use crate::gna_lib::BufferMap;

#[derive(Debug, Default)]
pub struct LayerConfiguration {
    pub buffers: BufferMap,
    // other config: activation lists, config lists
}

impl LayerConfiguration {
    pub fn new() -> Self { Self::default() }
}
