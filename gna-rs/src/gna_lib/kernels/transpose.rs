//! Simple, safe translations of transpose kernels from the original C++ sources.

/// Transpose an i16 matrix stored in row-major order.
/// `input` has length `rows * cols`, `output` must be the same length.
pub fn transpose_i16(input: &[i16], rows: usize, cols: usize, output: &mut [i16]) {
    assert_eq!(input.len(), rows * cols);
    assert_eq!(output.len(), rows * cols);

    for r in 0..rows {
        for c in 0..cols {
            output[c * rows + r] = input[r * cols + c];
        }
    }
}

/// Transpose an i8 matrix stored in row-major order.
pub fn transpose_i8(input: &[i8], rows: usize, cols: usize, output: &mut [i8]) {
    assert_eq!(input.len(), rows * cols);
    assert_eq!(output.len(), rows * cols);

    for r in 0..rows {
        for c in 0..cols {
            output[c * rows + r] = input[r * cols + c];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_i16_square() {
        let rows = 4usize;
        let cols = 4usize;
        let input: Vec<i16> = (0..(rows * cols)).map(|x| x as i16).collect();
        let mut out = vec![0i16; rows * cols];
        transpose_i16(&input, rows, cols, &mut out);
        for r in 0..rows {
            for c in 0..cols {
                assert_eq!(out[c * rows + r], input[r * cols + c]);
            }
        }
    }

    #[test]
    fn transpose_i16_rect() {
        let rows = 3usize;
        let cols = 5usize;
        let input: Vec<i16> = (0..(rows * cols)).map(|x| (x * 2) as i16).collect();
        let mut out = vec![0i16; rows * cols];
        transpose_i16(&input, rows, cols, &mut out);
        for r in 0..rows {
            for c in 0..cols {
                assert_eq!(out[c * rows + r], input[r * cols + c]);
            }
        }
    }

    #[test]
    fn transpose_i8_basic() {
        let rows = 2usize;
        let cols = 3usize;
        let input: Vec<i8> = (0..(rows * cols)).map(|x| (x as i8) - 10).collect();
        let mut out = vec![0i8; rows * cols];
        transpose_i8(&input, rows, cols, &mut out);
        for r in 0..rows {
            for c in 0..cols {
                assert_eq!(out[c * rows + r], input[r * cols + c]);
            }
        }
    }
}
