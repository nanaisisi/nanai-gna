/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::shape::Shape;
use crate::gna_lib::validator::Validator;

/// Simplified Rust port of the GNA Component helper.
#[derive(Debug, Clone)]
pub struct Component {
    pub dimensions: Shape,
    pub count: u32,
    pub component_index: u32,
    pub is_parameter: bool,
    pub validator: Option<Validator>,
}

impl Component {
    pub fn new(dimensions: Shape, component_index: u32, is_parameter: bool) -> Self {
        let count = dimensions.0.iter().product::<usize>() as u32;
        Self {
            dimensions,
            count,
            component_index,
            is_parameter,
            validator: None,
        }
    }

    pub fn with_validator(
        component: &Component,
        validator: Validator,
        _validate_dimensions: bool,
        component_index: u32,
        is_parameter: bool,
    ) -> Self {
        let mut result = component.clone();
        result.component_index = component_index;
        result.is_parameter = is_parameter;
        result.validator = Some(validator);
        result
    }

    pub fn from_validator(
        dimensions: Shape,
        validator: Validator,
        _validate_dimensions: bool,
        component_index: u32,
        is_parameter: bool,
    ) -> Self {
        let count = dimensions.0.iter().product::<usize>() as u32;
        Self {
            dimensions,
            count,
            component_index,
            is_parameter,
            validator: Some(validator),
        }
    }

    pub fn at(&self, dimension: usize) -> usize {
        self.dimensions.at(dimension)
    }

    pub fn validate(&self) -> bool {
        self.validator.as_ref().map_or(true, |v| v.validate())
    }

    pub fn validate_dimensions(&self) -> bool {
        true
    }

    pub fn expect_shape_is_valid(&self) -> bool {
        !self.dimensions.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_counts_dimensions() {
        let shape = Shape::with_dims(vec![2, 3, 4]);
        let component = Component::new(shape, 1, true);
        assert_eq!(component.count, 24);
    }

    #[test]
    fn component_at_returns_dimension_value() {
        let shape = Shape::with_dims(vec![5, 6]);
        let component = Component::new(shape, 0, false);
        assert_eq!(component.at(0), 5);
        assert_eq!(component.at(1), 6);
        assert_eq!(component.at(2), 0);
    }

    #[test]
    fn component_validate_returns_true_by_default() {
        let component = Component::new(Shape::with_dims(vec![1, 1]), 0, true);
        assert!(component.validate());
    }
}
