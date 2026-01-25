/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Minimal stub for Expect helpers (port of original Expect.h)

#[allow(dead_code)]
pub fn in_set<T: PartialEq>(_value: &T, _set: &[T]) -> bool { true }
