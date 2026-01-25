//! Skeleton for TransformMap and transform registration.

use crate::gna_lib::transform::BaseTransform;

#[derive(Debug, Default)]
pub struct TransformMap {
    list: Vec<BaseTransform>,
}

impl TransformMap {
    pub fn new() -> Self { Self::default() }
}
