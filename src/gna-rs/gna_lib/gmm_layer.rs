/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA `GmmLayer` helper.
#[derive(Debug, Default)]
pub struct GmmLayer;

impl GmmLayer {
    pub fn run(
        &self,
        input: &[u8],
        means: &[u8],
        input_length: usize,
        gaussian_constants: Option<&[u32]>,
        output: &mut [u32],
    ) {
        assert!(input_length > 0, "input length must be positive");
        assert_eq!(
            means.len() % input_length,
            0,
            "means length must be a multiple of input length"
        );

        let state_count = means.len() / input_length;
        assert_eq!(
            output.len(),
            state_count,
            "output length must equal GMM state count"
        );

        for state in 0..state_count {
            let base = state * input_length;
            let mut score = 0u32;

            for i in 0..input_length {
                score = score.wrapping_add(input[i] as u32 * means[base + i] as u32);
            }

            if let Some(constants) = gaussian_constants {
                score = score.wrapping_add(constants[state]);
            }

            output[state] = score;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmmLayer;

    #[test]
    fn gmm_layer_run_computes_simple_scores() {
        let layer = GmmLayer::default();
        let input = [2u8, 3, 4];
        let means = [1u8, 1, 1, 2, 2, 2];
        let mut output = [0u32; 2];

        layer.run(&input, &means, 3, None, &mut output);

        assert_eq!(output, [9, 18]);
    }

    #[test]
    fn gmm_layer_run_adds_gaussian_constants_when_provided() {
        let layer = GmmLayer::default();
        let input = [1u8, 2];
        let means = [2u8, 3, 4, 5];
        let constants = [10u32, 20u32];
        let mut output = [0u32; 2];

        layer.run(&input, &means, 2, Some(&constants), &mut output);

        assert_eq!(output, [18, 34]);
    }
}
