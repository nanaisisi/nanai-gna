/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/kernels/pwl.h / pwl.cpp

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PwlSegment {
    pub x_base: i32,
    pub y_base: i16,
    pub slope: i16,
}

pub fn pwl_eval(input: i16, segments: &[PwlSegment]) -> i16 {
    if segments.is_empty() {
        return input;
    }

    let x = input as i32;
    let selected = segments.iter().rev().find(|segment| x >= segment.x_base);

    if let Some(segment) = selected {
        let delta = x - segment.x_base;
        let shifted = ((delta as i64) * (segment.slope as i64)) >> 8;
        let result = (segment.y_base as i64) + shifted;
        result.clamp(i16::MIN as i64, i16::MAX as i16 as i64) as i16
    } else {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::{PwlSegment, pwl_eval};

    #[test]
    fn pwl_eval_returns_input_for_empty_segments() {
        assert_eq!(pwl_eval(10, &[]), 10);
    }

    #[test]
    fn pwl_eval_selects_highest_matching_segment() {
        let segments = [
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
        ];

        assert_eq!(pwl_eval(2, &segments), 2);
        assert_eq!(pwl_eval(4, &segments), 4);
        assert_eq!(pwl_eval(6, &segments), 8);
        assert_eq!(pwl_eval(8, &segments), 12);
    }
}
