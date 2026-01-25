//! Skeleton for SoftwareModel / SoftwareOnlyModel

#[derive(Debug)]
pub struct SoftwareModel {
    // software execution model
}

impl SoftwareModel {
    pub fn new() -> Self { Self {} }
}

#[derive(Debug)]
pub struct SoftwareOnlyModel(pub SoftwareModel);
