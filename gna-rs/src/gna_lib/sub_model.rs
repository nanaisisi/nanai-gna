/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Minimal Rust port of the GNA `SubModel` helper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubModelType {
    Software,
    Hardware,
    GmmHardware,
}

#[derive(Debug, Clone)]
pub struct SubModel {
    pub r#type: SubModelType,
    pub layer_index: u32,
    layer_count: u32,
}

impl SubModel {
    pub fn new(sub_model_type: SubModelType, layer_index: u32) -> Self {
        Self {
            r#type: sub_model_type,
            layer_index,
            layer_count: 1,
        }
    }

    pub fn add_layer(&mut self) {
        self.layer_count = self.layer_count.saturating_add(1);
    }

    pub fn contains(&self, layer_index: u32) -> bool {
        layer_index >= self.layer_index && layer_index < self.layer_index + self.layer_count
    }

    pub fn get_layer_count(&self) -> u32 {
        self.layer_count
    }

    pub fn is_software_layer(layer_index: u32, sub_models: &[SubModel]) -> bool {
        sub_models.iter().any(|sub_model| {
            sub_model.contains(layer_index) && sub_model.r#type == SubModelType::Software
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{SubModel, SubModelType};

    #[test]
    fn sub_model_add_layer_increments_count() {
        let mut model = SubModel::new(SubModelType::Hardware, 2);
        assert_eq!(model.get_layer_count(), 1);

        model.add_layer();
        assert_eq!(model.get_layer_count(), 2);
    }

    #[test]
    fn sub_model_contains_layer_within_range() {
        let mut model = SubModel::new(SubModelType::Hardware, 2);
        model.add_layer();

        assert!(model.contains(2));
        assert!(model.contains(3));
        assert!(!model.contains(4));
    }

    #[test]
    fn is_software_layer_returns_true_when_match_found() {
        let mut software = SubModel::new(SubModelType::Software, 0);
        software.add_layer();
        let hardware = SubModel::new(SubModelType::Hardware, 2);

        assert!(SubModel::is_software_layer(
            1,
            &[software.clone(), hardware]
        ));
    }

    #[test]
    fn is_software_layer_returns_false_when_no_software_match() {
        let software = SubModel::new(SubModelType::Software, 0);
        let hardware = SubModel::new(SubModelType::Hardware, 1);

        assert!(!SubModel::is_software_layer(2, &[software, hardware]));
    }
}
