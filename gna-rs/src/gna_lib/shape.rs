/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::layout::Layout;

/// Simplified Rust port of the GNA Shape helper.
///
/// This holds a list of tensor dimensions plus an optional layout description.
#[derive(Debug, Default, Clone)]
pub struct Shape(pub Vec<usize>, pub Layout);

impl Shape {
    pub fn new() -> Self {
        Self(Vec::new(), Layout::new())
    }

    pub fn with_dims(dimensions: Vec<usize>) -> Self {
        Self(dimensions, Layout::new())
    }

    pub fn with_dims_and_layout(dimensions: Vec<usize>, layout: Layout) -> Self {
        if !layout.validate_number_of_dimensions(dimensions.len()) {
            Self(dimensions, Layout::new())
        } else {
            Self(dimensions, layout)
        }
    }

    pub fn at(&self, index: usize) -> usize {
        self.0.get(index).copied().unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_number_of_elements(&self) -> usize {
        if self.0.is_empty() || self.0.iter().any(|&dim| dim == 0) {
            return 0;
        }
        self.0.iter().product()
    }

    pub fn reshape(&self, new_layout: &Layout) -> Self {
        if new_layout.validate_number_of_dimensions(self.0.len()) {
            let mut result = self.clone();
            result.1 = new_layout.clone();
            result
        } else {
            self.clone()
        }
    }

    pub fn expect_fits(&self, envelope: &Shape) -> bool {
        if self.len() != envelope.len() {
            return false;
        }
        self.0
            .iter()
            .zip(envelope.0.iter())
            .all(|(&dim, &limit)| dim <= limit)
    }

    pub fn expect_equal(&self, reference: &Shape) -> bool {
        self.0 == reference.0
    }

    pub fn is_square(&self) -> bool {
        if self.0.len() <= 1 {
            return true;
        }
        let first = self.0[0];
        self.0.iter().all(|&dim| dim == first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_get_number_of_elements_returns_product() {
        let shape = Shape::with_dims(vec![2, 3, 4]);
        assert_eq!(shape.get_number_of_elements(), 24);
    }

    #[test]
    fn shape_get_number_of_elements_returns_zero_for_empty_or_zero_dims() {
        let shape = Shape::with_dims(vec![]);
        assert_eq!(shape.get_number_of_elements(), 0);
        let shape = Shape::with_dims(vec![2, 0, 4]);
        assert_eq!(shape.get_number_of_elements(), 0);
    }

    #[test]
    fn shape_reshape_updates_layout_when_compatible() {
        let shape = Shape::with_dims_and_layout(vec![1, 2, 3, 4], Layout::from_str("NCHW"));
        let reshaped = shape.reshape(&Layout::from_str("NHWC"));
        assert_eq!(reshaped.1, Layout::from_str("NHWC"));
        assert_eq!(reshaped.0, vec![1, 2, 3, 4]);
    }

    #[test]
    fn shape_reshape_preserves_shape_when_incompatible_order() {
        let shape = Shape::with_dims_and_layout(vec![1, 2, 3], Layout::from_str("NCHW"));
        let reshaped = shape.reshape(&Layout::from_str("NHWC"));
        assert_eq!(reshaped.1, Layout::new());
    }

    #[test]
    fn shape_expect_fits_checks_dimension_bounds() {
        let shape = Shape::with_dims(vec![2, 3, 4]);
        let envelope = Shape::with_dims(vec![2, 3, 5]);
        assert!(shape.expect_fits(&envelope));
        let smaller = Shape::with_dims(vec![2, 4, 4]);
        assert!(!smaller.expect_fits(&envelope));
    }

    #[test]
    fn shape_expect_equal_requires_same_dimensions() {
        let shape = Shape::with_dims(vec![2, 3]);
        let same = Shape::with_dims(vec![2, 3]);
        let different = Shape::with_dims(vec![3, 2]);
        assert!(shape.expect_equal(&same));
        assert!(!shape.expect_equal(&different));
    }

    #[test]
    fn shape_is_square_checks_equal_dimensions() {
        assert!(Shape::with_dims(vec![3, 3, 3]).is_square());
        assert!(!Shape::with_dims(vec![3, 4, 3]).is_square());
    }
}
