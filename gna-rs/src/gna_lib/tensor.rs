//! Skeleton for `Tensor` type.

#[derive(Debug, Default, Clone)]
pub struct Tensor {
    pub size: usize,
}

impl Tensor {
    pub fn new(size: usize) -> Self { Self { size } }
}
