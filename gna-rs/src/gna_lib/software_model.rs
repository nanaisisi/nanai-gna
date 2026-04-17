/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for SoftwareModel / SoftwareOnlyModel

#[derive(Debug)]
pub struct SoftwareModel {
    // software execution model
}

impl SoftwareModel {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::SoftwareModel;

    #[test]
    fn software_model_new_creates_instance() {
        let _model = SoftwareModel::new();
    }
}
