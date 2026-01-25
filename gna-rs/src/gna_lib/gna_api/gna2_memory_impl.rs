// Auto-generated Rust stub for original: gna/src/gna-lib/gna-api/gna2-memory-impl.h / .cpp
// SPDX-License-Identifier: LGPL-2.1-or-later

#![allow(dead_code)]

/// Memory API stubs

pub fn memory_alloc(_size: usize) -> Option<usize> { Some(0) }
pub fn memory_free(_addr: usize) -> bool { true }
pub fn memory_set_tag(_addr: usize, _tag: u32) -> bool { true }
