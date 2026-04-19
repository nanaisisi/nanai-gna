/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Minimal PWL activation implementation for GNA porting.
///
/// This is a simplified piecewise-linear evaluator intended to capture
/// the shape of the original GNA activation segments.
use crate::gna_lib::kernels::pwl::{PwlSegment, pwl_eval};

#[derive(Debug, Clone)]
pub struct ActivationFunction {
    segments: Vec<PwlSegment>,
}

impl ActivationFunction {
    pub fn new(segments: Vec<PwlSegment>) -> Self {
        Self { segments }
    }

    pub fn apply(&self, data: &mut [i16]) {
        for value in data.iter_mut() {
            *value = self.evaluate(*value);
        }
    }

    pub fn evaluate(&self, input: i16) -> i16 {
        pwl_eval(input, &self.segments)
    }
}

#[cfg(test)]
mod tests {
    use super::ActivationFunction;
    use crate::gna_lib::kernels::pwl::PwlSegment;

    #[test]
    fn activation_function_applies_linear_pwl_segments() {
        let function = ActivationFunction::new(vec![
            PwlSegment {
                x_base: 0,
                y_base: 0,
                slope: 256,
            },
            PwlSegment {
                x_base: 4,
                y_base: 4,
                slope: 512,
            },
        ]);

        let mut values = [0i16, 2, 4, 6, 8];
        function.apply(&mut values);

        assert_eq!(values, [0, 2, 4, 8, 12]);
    }

    #[test]
    fn activation_function_returns_input_when_no_segment_matches() {
        let function = ActivationFunction::new(vec![PwlSegment {
            x_base: 10,
            y_base: 1,
            slope: 256,
        }]);
        let mut values = [0i16, 5, 9];

        function.apply(&mut values);

        assert_eq!(values, [0, 5, 9]);
    }
}
