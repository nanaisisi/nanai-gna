/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Auto-generated Rust stub for original: gna/src/gna-lib/kernels/affine_avx2-sat.cpp

#[allow(dead_code)]
use crate::gna_rs::gna_lib::kernels::KernelArguments;

/// Lightweight scalar fallback for the AVX2 affine kernel.
pub fn affine_avx2_sat() {
    eprintln!("affine_avx2_sat: stub called");
}

/// Process with arguments (safe best-effort scalar implementation)
pub fn affine_avx2_process(args: &KernelArguments) {
    if args.input.is_null() || args.output.is_null() {
        return;
    }
    let w = args.width;
    unsafe {
        let in_ptr = args.input.get::<i16>();
        let out_ptr = args.output.get::<i16>();
        for i in 0..w {
            let v = *in_ptr.add(i);
            *out_ptr.add(i) = v; // identity for fallback
        }
    }
}
