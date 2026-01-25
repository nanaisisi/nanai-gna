/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Minimal translation of `Macros.h` constants/macros used across the codebase.

/// Helpful max/min helpers (Rust has std already, this mirrors intent)
pub const fn max_u32(a: u32, b: u32) -> u32 { if a > b { a } else { b } }
pub const fn min_u32(a: u32, b: u32) -> u32 { if a < b { a } else { b } }

// Add other macro constants as needed when porting
