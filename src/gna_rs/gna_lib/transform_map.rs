/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for TransformMap and transform registration.
use crate::gna_rs::gna_lib::transform::{BaseTransform, TransformOperation};

#[derive(Debug, Default, Clone)]
pub struct TransformMap {
    list: Vec<BaseTransform>,
}

impl TransformMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emplace(&mut self, transform: BaseTransform) -> &BaseTransform {
        if self.get_optional(transform.operation()).is_some() {
            panic!("duplicate transform operation: {:?}", transform.operation());
        }

        self.list.push(transform);
        self.list
            .last()
            .expect("TransformMap should contain the emplaced transform")
    }

    pub fn get_optional(&self, operation: TransformOperation) -> Option<&BaseTransform> {
        self.list
            .iter()
            .find(|entry| entry.operation() == operation)
    }

    pub fn get(&self, operation: TransformOperation) -> &BaseTransform {
        self.get_optional(operation)
            .expect("Transform operation not found in TransformMap")
    }

    pub fn contains(&self, operation: TransformOperation) -> bool {
        self.get_optional(operation).is_some()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &BaseTransform> {
        self.list.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseTransform, TransformMap, TransformOperation};

    #[test]
    fn transform_map_can_emplace_and_retrieve_transforms() {
        let mut map = TransformMap::new();
        let transform = BaseTransform::new(TransformOperation::Affine);
        map.emplace(transform);

        assert!(map.contains(TransformOperation::Affine));
        let retrieved = map.get(TransformOperation::Affine);
        assert_eq!(retrieved.operation(), TransformOperation::Affine);
    }

    #[test]
    fn transform_map_get_optional_returns_none_for_missing_operation() {
        let map = TransformMap::new();
        assert!(map.get_optional(TransformOperation::Gmm).is_none());
    }

    #[test]
    #[should_panic(expected = "duplicate transform operation")]
    fn transform_map_emplace_panics_on_duplicate_operation() {
        let mut map = TransformMap::new();
        let transform = BaseTransform::new(TransformOperation::Affine);
        map.emplace(transform);
        map.emplace(BaseTransform::new(TransformOperation::Affine));
    }
}
