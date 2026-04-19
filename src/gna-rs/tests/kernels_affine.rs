use gna_rs::gna_lib::kernels::*;
use gna_rs::gna_lib::kernels::KernelArguments;
use gna_rs::common::BaseAddress;

#[test]
fn affine_sse4_process_copies_data() {
    let mut src: [i16; 4] = [1, -2, 300, -400];
    let mut dst: [i16; 4] = [0; 4];
    let args = KernelArguments { input: BaseAddress::from_ptr(src.as_mut_ptr() as *mut u8), output: BaseAddress::from_ptr(dst.as_mut_ptr() as *mut u8), aux: None, width: 4, height: 1 };
    affine_sse4_sat::affine_sse4_process(&args);
    assert_eq!(dst, src);
}

#[test]
fn affine_avx2_process_copies_data() {
    let mut src: [i16; 4] = [10, 20, -30, 40];
    let mut dst: [i16; 4] = [0; 4];
    let args = KernelArguments { input: BaseAddress::from_ptr(src.as_mut_ptr() as *mut u8), output: BaseAddress::from_ptr(dst.as_mut_ptr() as *mut u8), aux: None, width: 4, height: 1 };
    affine_avx2_sat::affine_avx2_process(&args);
    assert_eq!(dst, src);
}