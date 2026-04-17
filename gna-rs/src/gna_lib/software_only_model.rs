/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::software_model::SoftwareModel;

/// Stubbed Rust port of the original GNA `SoftwareOnlyModel` helper.
#[derive(Debug)]
pub struct SoftwareOnlyModel(pub SoftwareModel);

impl SoftwareOnlyModel {
    pub fn new() -> Self {
        Self(SoftwareModel::new())
    }

    pub fn is_software_only(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::SoftwareOnlyModel;

    #[test]
    fn software_only_model_new_creates_instance() {
        let model = SoftwareOnlyModel::new();
        assert!(model.is_software_only());
    }
}
