/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA AffineFunctions helper.
///
/// This implementation provides minimal affine evaluation logic for
/// elementwise weight application plus bias, suitable for early porting
/// stages and unit testing.
#[derive(Debug, Default)]
pub struct AffineFunctions;

impl AffineFunctions {
    pub fn compute(&self, input: &[i16], weights: &[i16], bias: i16, output: &mut [i16]) {
        assert_eq!(
            input.len(),
            weights.len(),
            "input and weights must match length"
        );
        assert_eq!(output.len(), input.len(), "output must match input length");

        for ((value, weight), slot) in input.iter().zip(weights.iter()).zip(output.iter_mut()) {
            *slot = value.wrapping_mul(*weight).wrapping_add(bias);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AffineFunctions;

    #[test]
    fn affine_functions_compute_applies_affine_transform() {
        let aff = AffineFunctions::default();
        let input = [1i16, 2, 3];
        let weights = [2i16, 3, 4];
        let bias = 1i16;
        let mut output = [0i16; 3];

        aff.compute(&input, &weights, bias, &mut output);

        assert_eq!(output, [3, 7, 13]);
    }
}
