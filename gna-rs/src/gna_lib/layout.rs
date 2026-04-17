/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA `Layout` helper.
///
/// This version works with common tensor order strings such as `NCHW` and `NHWC`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout(String);

impl Layout {
    pub const ANY_LAYOUT: &'static str = "LAYOUT_ANY";

    pub fn new() -> Self {
        Self(Self::ANY_LAYOUT.to_string())
    }

    pub fn from_str(layout: &str) -> Self {
        Self(layout.to_string())
    }

    pub fn is_any(&self) -> bool {
        self.0 == Self::ANY_LAYOUT
    }

    pub fn validate_number_of_dimensions(&self, shape_dimensions: usize) -> bool {
        self.is_any() || self.0.len() == shape_dimensions
    }

    pub fn reshape(&mut self, new_layout: &Layout, shape_dimensions: usize) {
        if new_layout.validate_number_of_dimensions(shape_dimensions) {
            self.0 = new_layout.0.clone();
        }
    }

    pub fn get_api_index(&self, dim: char) -> i32 {
        if self.is_any() {
            return -1;
        }
        self.0
            .chars()
            .position(|c| c == dim)
            .map(|idx| idx as i32)
            .unwrap_or(-1)
    }

    pub fn get_api_index_char(&self, dim: char) -> i32 {
        self.get_api_index(dim)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Layout;

    #[test]
    fn layout_default_is_any() {
        let layout = Layout::new();
        assert!(layout.is_any());
        assert!(layout.validate_number_of_dimensions(0));
        assert_eq!(layout.get_api_index('N'), -1);
    }

    #[test]
    fn layout_validate_number_of_dimensions_for_known_layout() {
        let layout = Layout::from_str("NCHW");
        assert!(!layout.is_any());
        assert!(layout.validate_number_of_dimensions(4));
        assert!(!layout.validate_number_of_dimensions(3));
    }

    #[test]
    fn layout_gets_api_index_for_dimension() {
        let layout = Layout::from_str("NHWC");
        assert_eq!(layout.get_api_index('N'), 0);
        assert_eq!(layout.get_api_index('H'), 1);
        assert_eq!(layout.get_api_index('W'), 2);
        assert_eq!(layout.get_api_index('Z'), -1);
    }

    #[test]
    fn layout_reshape_updates_layout_when_dimensions_match() {
        let mut layout = Layout::from_str("NCHW");
        let target = Layout::from_str("NHWC");

        layout.reshape(&target, 4);

        assert_eq!(layout, target);
    }
}
