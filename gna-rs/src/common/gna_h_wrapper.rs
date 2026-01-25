/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
//! Wrapper for C++/C headers (`gna-h-wrapper.h`) - currently a doc-only placeholder.

// The original header consolidates platform-specific includes and helper macros.
// When adding FFI bindings, implement them here (behind `ffi` feature).

#[cfg(feature = "ffi")]
mod ffi {
    // pub(crate) unsafe fn call_some_c_api() { /* ... */ }
}
