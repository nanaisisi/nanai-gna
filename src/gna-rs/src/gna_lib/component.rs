/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::parameter_limits::{ComponentLimits, RangeLimits, ShapeLimits};
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
        let count = dimensions.get_number_of_elements() as u32;
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
        let count = dimensions.get_number_of_elements() as u32;
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

    pub fn validate_with_limits(
        &self,
        limits: &ComponentLimits,
        validate_dimensions: bool,
    ) -> bool {
        if validate_dimensions {
            if self.dimensions.len() != limits.dimensions.len() {
                return false;
            }
            for (index, limit) in limits.dimensions.iter().enumerate() {
                let dim = self.dimensions.at(index) as u32;
                if !self.dimension_is_valid(dim, limit) {
                    return false;
                }
            }
        }
        true
    }

    pub fn validate_dimensions(&self) -> bool {
        self.dimensions.len() > 0
    }

    pub fn expect_shape_is_valid(&self) -> bool {
        self.dimensions.len() > 0 && self.dimensions.0.iter().all(|&dim| dim > 0)
    }

    fn dimension_is_valid(&self, dimension: u32, limits: &RangeLimits<u32>) -> bool {
        if dimension < limits.min.value {
            return false;
        }
        if dimension > limits.max.value {
            return false;
        }
        true
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

    #[test]
    fn component_validate_with_limits_rejects_wrong_length() {
        let shape = Shape::with_dims(vec![2, 3]);
        let component = Component::new(shape, 0, true);
        let limits = ComponentLimits::new(
            crate::gna_lib::parameter_limits::OrderLimits::new(0, 0),
            vec![RangeLimits {
                min: crate::gna_lib::parameter_limits::ValueLimits::new(1, 0),
                max: crate::gna_lib::parameter_limits::ValueLimits::new(5, 0),
            }],
        );
        assert!(!component.validate_with_limits(&limits, true));
    }

    #[test]
    fn component_validate_with_limits_accepts_valid_dimensions() {
        let shape = Shape::with_dims(vec![2, 3]);
        let component = Component::new(shape, 0, true);
        let limits = ComponentLimits::new(
            crate::gna_lib::parameter_limits::OrderLimits::new(0, 0),
            vec![
                RangeLimits {
                    min: crate::gna_lib::parameter_limits::ValueLimits::new(1, 0),
                    max: crate::gna_lib::parameter_limits::ValueLimits::new(5, 0),
                },
                RangeLimits {
                    min: crate::gna_lib::parameter_limits::ValueLimits::new(1, 0),
                    max: crate::gna_lib::parameter_limits::ValueLimits::new(5, 0),
                },
            ],
        );
        assert!(component.validate_with_limits(&limits, true));
    }

    #[test]
    fn component_expect_shape_is_valid_rejects_empty_shape() {
        let component = Component::new(Shape::with_dims(vec![]), 0, true);
        assert!(!component.expect_shape_is_valid());
    }
}
