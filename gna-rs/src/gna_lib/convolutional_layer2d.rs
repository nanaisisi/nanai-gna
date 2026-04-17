/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::activation_function::ActivationFunction;
use crate::gna_lib::convolutional_functions2d::ConvolutionalFunctions2D;

/// Minimal Rust port of the GNA `ConvolutionalLayer2D` helper.
#[derive(Debug, Default)]
pub struct ConvolutionalLayer2D;

impl ConvolutionalLayer2D {
    pub fn run(
        &self,
        input: &[i16],
        input_width: usize,
        input_height: usize,
        filters: &[i16],
        filter_width: usize,
        filter_height: usize,
        bias: i16,
        activation: Option<&ActivationFunction>,
        output: &mut [i16],
    ) {
        let mut intermediate = vec![0i16; output.len()];

        ConvolutionalFunctions2D::new().conv2d(
            input,
            input_width,
            input_height,
            filters,
            filter_width,
            filter_height,
            &mut intermediate,
        );

        for (dest, value) in output.iter_mut().zip(intermediate.into_iter()) {
            *dest = value.wrapping_add(bias);
        }

        if let Some(activation_fn) = activation {
            activation_fn.apply(output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConvolutionalLayer2D;
    use crate::gna_lib::activation_function::ActivationFunction;
    use crate::gna_lib::kernels::pwl::PwlSegment;

    #[test]
    fn convolutional_layer2d_run_applies_conv_bias_and_activation() {
        let layer = ConvolutionalLayer2D::default();
        let input = [1i16, 2, 3, 4, 5, 6, 7, 8, 9];
        let filters = [1i16, 0, 0, 1];
        let mut output = [0i16; 4];
        let activation = ActivationFunction::new(vec![PwlSegment {
            x_base: 0,
            y_base: 0,
            slope: 256,
        }]);

        layer.run(
            &input,
            3,
            3,
            &filters,
            2,
            2,
            1,
            Some(&activation),
            &mut output,
        );

        assert_eq!(output, [7, 9, 13, 15]);
    }
}
