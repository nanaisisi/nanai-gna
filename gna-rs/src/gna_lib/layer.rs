//! Skeleton for `Layer` and related types.

use crate::gna_lib::buffer_map::BufferMap;

#[derive(Debug)]
pub struct Layer {
    // validator, transforms, input/output tensors
    pub buffers: BufferMap,
}

impl Layer {
    pub fn new() -> Self { Self { buffers: BufferMap::new() } }
}
