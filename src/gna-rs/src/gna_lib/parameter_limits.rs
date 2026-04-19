/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::common_api::Gna2Status;

/// Value limits with an associated status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueLimits<T> {
    pub value: T,
    pub error: Gna2Status,
}

impl<T> ValueLimits<T> {
    pub fn new(value: T, error: Gna2Status) -> Self {
        Self { value, error }
    }
}

/// Alignment limits used for pointer validation.
pub type AlignLimits = ValueLimits<u32>;

/// Shape limits keyed by dimension index.
pub type ShapeLimits = Vec<RangeLimits<u32>>;

/// Order limits used for tensor order validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderLimits {
    pub value: u32,
    pub error: Gna2Status,
}

impl OrderLimits {
    pub fn new(value: u32, error: Gna2Status) -> Self {
        Self { value, error }
    }
}

/// Component limits used when validating a component shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentLimits {
    pub order: OrderLimits,
    pub dimensions: ShapeLimits,
}

impl ComponentLimits {
    pub fn new(order: OrderLimits, dimensions: ShapeLimits) -> Self {
        Self { order, dimensions }
    }
}

/// Range limits used for parameter validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeLimits<T> {
    pub min: ValueLimits<T>,
    pub max: ValueLimits<T>,
}

impl<T> RangeLimits<T> {
    pub fn new(min: ValueLimits<T>, max: ValueLimits<T>) -> Self {
        Self { min, max }
    }
}

/// Set limits used for validation against an explicit list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLimits<T> {
    pub values: Vec<T>,
    pub error: Gna2Status,
}

impl<T> SetLimits<T> {
    pub fn new(values: Vec<T>, error: Gna2Status) -> Self {
        Self { values, error }
    }
}

#[allow(dead_code)]
pub struct ParameterLimits;

impl ParameterLimits {
    pub fn validate(&self) -> bool {
        true
    }
}
