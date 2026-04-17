/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_lib::activation_function::ActivationFunction;
use crate::gna_lib::convolutional_functions::ConvolutionalFunctions;

/// Minimal Rust port of the GNA `ConvolutionalLayer` helper.
///
/// This implementation uses the ported `ConvolutionalFunctions` helper for the
/// core convolution and adds bias plus optional activation.
#[derive(Debug, Default)]
pub struct ConvolutionalLayer;

impl ConvolutionalLayer {
    pub fn run(
        &self,
        input: &[i16],
        input_width: usize,
        input_height: usize,
        kernel: &[i16],
        kernel_size: usize,
        bias: i16,
        activation: Option<&ActivationFunction>,
        output: &mut [i16],
    ) {
        let mut intermediate = vec![0i16; output.len()];
        ConvolutionalFunctions::default().conv2d(
            input,
            input_width,
            input_height,
            kernel,
            kernel_size,
            &mut intermediate,
        );

        for (slot, value) in output.iter_mut().zip(intermediate.into_iter()) {
            *slot = value.wrapping_add(bias);
        }

        if let Some(act) = activation {
            act.apply(output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConvolutionalLayer;
    use crate::gna_lib::activation_function::ActivationFunction;
    use crate::gna_lib::kernels::pwl::PwlSegment;

    #[test]
    fn convolutional_layer_run_applies_conv_bias_and_activation() {
        let layer = ConvolutionalLayer::default();
        let input = [1i16, 2, 3, 4, 5, 6, 7, 8, 9];
        let kernel = [1i16, 0, 0, 1];
        let mut output = [0i16; 4];
        let activation = ActivationFunction::new(vec![PwlSegment {
            x_base: 0,
            y_base: 0,
            slope: 256,
        }]);

        layer.run(&input, 3, 3, &kernel, 2, 1, Some(&activation), &mut output);

        assert_eq!(output, [7, 9, 13, 15]);
    }
}
