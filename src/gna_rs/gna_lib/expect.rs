/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::common_api::Gna2Status;
use crate::gna_rs::gna_lib::parameter_limits::{AlignLimits, RangeLimits, SetLimits};

const GNA2_STATUS_NULL_ARGUMENT_NOT_ALLOWED: Gna2Status = -6;
const GNA2_STATUS_NOT_MULTIPLE_OF: Gna2Status = -14;
const GNA2_STATUS_XNN_ERROR_INVALID_BUFFER: Gna2Status = -15;

/// Port of the GNA Expect validation helper.
#[allow(dead_code)]
pub struct Expect;

impl Expect {
    pub fn true_(condition: bool, error: Gna2Status) {
        if !condition {
            panic!("GNA Expect::True failed: status {}", error);
        }
    }

    pub fn false_(condition: bool, error: Gna2Status) {
        Self::true_(!condition, error);
    }

    pub fn equal<T: PartialEq>(a: T, b: T, error: Gna2Status) {
        Self::true_(a == b, error);
    }

    pub fn one<T: PartialEq + From<u8>>(value: T, error: Gna2Status) {
        Self::true_(value == T::from(1u8), error);
    }

    pub fn zero<T: PartialEq + From<u8>>(value: T, error: Gna2Status) {
        Self::true_(value == T::from(0u8), error);
    }

    pub fn gt_zero<T: PartialOrd + From<u8>>(value: T, error: Gna2Status) {
        Self::true_(value > T::from(0u8), error);
    }

    pub fn success(status: Gna2Status) {
        Self::true_(status == 0, status);
    }

    pub fn not_null<T>(pointer: *const T, error: Gna2Status) {
        Self::false_(pointer.is_null(), error);
    }

    pub fn null<T>(pointer: *const T, error: Gna2Status) {
        Self::true_(pointer.is_null(), error);
    }

    pub fn aligned_to(pointer: *const u8, align_limits: AlignLimits) {
        let address = pointer as usize;
        Self::true_(
            address % (align_limits.value as usize) == 0,
            align_limits.error,
        );
    }

    pub fn aligned_to_default(pointer: *const u8, alignment: u32) {
        Self::aligned_to(
            pointer,
            AlignLimits::new(alignment, GNA2_STATUS_XNN_ERROR_INVALID_BUFFER),
        );
    }

    pub fn valid_buffer(pointer: *const u8, alignment: u32) {
        Self::not_null(pointer, GNA2_STATUS_NULL_ARGUMENT_NOT_ALLOWED);
        Self::aligned_to_default(pointer, alignment);
    }

    pub fn in_memory_range(
        buffer: *const u8,
        buffer_size: usize,
        memory: *const u8,
        memory_size: usize,
    ) -> bool {
        let buffer_end = unsafe { buffer.add(buffer_size) };
        let memory_end = unsafe { memory.add(memory_size) };
        (buffer >= memory) && (buffer_end <= memory_end)
    }

    pub fn valid_boundaries(
        buffer: *const u8,
        buffer_size: usize,
        memory: *const u8,
        memory_size: usize,
    ) {
        Self::false_(
            Self::in_memory_range(buffer, buffer_size, memory, memory_size),
            GNA2_STATUS_XNN_ERROR_INVALID_BUFFER,
        );
    }

    pub fn multiplicity_of<T>(parameter: T, multiplicity: T, error: Gna2Status)
    where
        T: std::ops::Rem<Output = T> + PartialEq + Copy + From<u8>,
    {
        Self::true_(parameter % multiplicity == T::from(0u8), error);
    }

    pub fn multiplicity_of_default<T>(parameter: T, multiplicity: T)
    where
        T: std::ops::Rem<Output = T> + PartialEq + Copy + From<u8>,
    {
        Self::multiplicity_of(parameter, multiplicity, GNA2_STATUS_NOT_MULTIPLE_OF);
    }

    pub fn in_range<T>(parameter: T, max: T, error: Gna2Status)
    where
        T: PartialOrd + Copy + From<u8>,
    {
        Self::in_range_between(parameter, T::from(0u8), max, error);
    }

    pub fn in_range_between<T>(parameter: T, a: T, b: T, error: Gna2Status)
    where
        T: PartialOrd + Copy,
    {
        Self::false_(parameter < a, error);
        Self::false_(parameter > b, error);
    }

    pub fn in_range_limits<T>(parameter: T, limit: RangeLimits<T>)
    where
        T: PartialOrd + Copy,
    {
        Self::in_range_between(parameter, limit.min.value, limit.max.value, limit.min.error);
    }

    pub fn in_set<T: PartialEq>(parameter: T, set_limits: SetLimits<T>) {
        for item in set_limits.values.iter() {
            if *item == parameter {
                return;
            }
        }
        panic!("GNA Expect::InSet failed: status {}", set_limits.error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_true_does_not_panic_when_condition_is_met() {
        Expect::true_(true, 0);
    }

    #[test]
    #[should_panic]
    fn expect_true_panics_when_condition_is_not_met() {
        Expect::true_(false, 123);
    }

    #[test]
    fn expect_not_null_does_not_panic_for_valid_pointer() {
        let x = 42u32;
        Expect::not_null(&x as *const u32, GNA2_STATUS_NULL_ARGUMENT_NOT_ALLOWED);
    }

    #[test]
    #[should_panic]
    fn expect_not_null_panics_for_null_pointer() {
        Expect::not_null(
            std::ptr::null::<u32>(),
            GNA2_STATUS_NULL_ARGUMENT_NOT_ALLOWED,
        );
    }

    #[test]
    fn expect_in_range_accepts_value_at_boundaries() {
        Expect::in_range_between(5u32, 0u32, 5u32, 1);
    }

    #[test]
    #[should_panic]
    fn expect_in_range_panics_for_out_of_range() {
        Expect::in_range_between(10u32, 0u32, 5u32, 1);
    }

    #[test]
    fn expect_in_set_accepts_valid_value() {
        let set = SetLimits::new(vec![1, 2, 3], 42);
        Expect::in_set(2, set);
    }

    #[test]
    #[should_panic]
    fn expect_in_set_panics_for_invalid_value() {
        let set = SetLimits::new(vec![1, 2, 3], 42);
        Expect::in_set(4, set);
    }
}
