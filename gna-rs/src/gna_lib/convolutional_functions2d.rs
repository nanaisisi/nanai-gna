/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA ConvolutionalFunctions2D helper.
#[derive(Debug, Default)]
pub struct ConvolutionalFunctions2D;

impl ConvolutionalFunctions2D {
    pub fn new() -> Self {
        Self {}
    }

    pub fn conv2d(
        &self,
        input: &[i16],
        input_width: usize,
        input_height: usize,
        kernel: &[i16],
        kernel_width: usize,
        kernel_height: usize,
        output: &mut [i16],
    ) {
        assert_eq!(
            kernel.len(),
            kernel_width * kernel_height,
            "kernel length is invalid"
        );
        let output_width = input_width
            .checked_sub(kernel_width)
            .expect("kernel width must be <= input width")
            + 1;
        let output_height = input_height
            .checked_sub(kernel_height)
            .expect("kernel height must be <= input height")
            + 1;
        assert_eq!(
            output.len(),
            output_width * output_height,
            "output buffer length is invalid"
        );

        for y in 0..output_height {
            for x in 0..output_width {
                let mut sum = 0i16;
                for ky in 0..kernel_height {
                    for kx in 0..kernel_width {
                        let in_x = x + kx;
                        let in_y = y + ky;
                        let input_index = in_y * input_width + in_x;
                        let kernel_index = ky * kernel_width + kx;
                        sum =
                            sum.wrapping_add(input[input_index].wrapping_mul(kernel[kernel_index]));
                    }
                }
                output[y * output_width + x] = sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConvolutionalFunctions2D;

    #[test]
    fn convolutional_functions2d_conv2d_computes_valid_output() {
        let conv2d = ConvolutionalFunctions2D::new();
        let input = [1i16, 2, 3, 4, 5, 6, 7, 8, 9];
        let kernel = [1i16, 0, 0, 1];
        let mut output = [0i16; 4];

        conv2d.conv2d(&input, 3, 3, &kernel, 2, 2, &mut output);

        assert_eq!(output, [6, 8, 12, 14]);
    }
}
