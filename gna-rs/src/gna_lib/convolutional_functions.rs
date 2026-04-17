/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA ConvolutionalFunctions helper.
///
/// Provides a small 2D convolution implementation for unit testing and
/// early ported validation.
#[derive(Debug, Default)]
pub struct ConvolutionalFunctions;

impl ConvolutionalFunctions {
    pub fn conv2d(
        &self,
        input: &[i16],
        input_width: usize,
        input_height: usize,
        kernel: &[i16],
        kernel_size: usize,
        output: &mut [i16],
    ) {
        let output_width = input_width
            .checked_sub(kernel_size)
            .expect("kernel width must be <= input width")
            + 1;
        let output_height = input_height
            .checked_sub(kernel_size)
            .expect("kernel height must be <= input height")
            + 1;

        assert_eq!(
            kernel.len(),
            kernel_size * kernel_size,
            "kernel must be square with kernel_size^2 elements"
        );
        assert_eq!(
            output.len(),
            output_width * output_height,
            "output buffer has unexpected length"
        );

        for y in 0..output_height {
            for x in 0..output_width {
                let mut sum = 0i16;
                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        let in_x = x + kx;
                        let in_y = y + ky;
                        let in_index = in_y * input_width + in_x;
                        let k_index = ky * kernel_size + kx;
                        sum = sum.wrapping_add(input[in_index].wrapping_mul(kernel[k_index]));
                    }
                }
                output[y * output_width + x] = sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConvolutionalFunctions;

    #[test]
    fn convolutional_functions_conv2d_computes_valid_output() {
        let conv = ConvolutionalFunctions::default();
        let input = [1i16, 2, 3, 4, 5, 6, 7, 8, 9];
        let kernel = [1i16, 0, -1, 1];
        let mut output = [0i16; 4];

        conv.conv2d(&input, 3, 3, &kernel, 2, &mut output);

        assert_eq!(output, [2, 3, 5, 6]);
    }
}
