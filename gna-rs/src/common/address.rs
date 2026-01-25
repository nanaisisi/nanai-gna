/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
//! Simple portability shim for `BaseAddress` (from the original `Address.h`).

/// Minimal representation of a BaseAddress used by ported code.
/// This intentionally keeps a raw pointer; caller is responsible for safety.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BaseAddress {
    ptr: *mut u8,
}

// BaseAddress is a thin wrapper around a raw pointer. Declare it Send+Sync so it can
// be safely shared across threads in our simplified skeleton (original code uses
// raw pointers in concurrent structures with appropriate care).
unsafe impl Send for BaseAddress {}
unsafe impl Sync for BaseAddress {}

impl BaseAddress {
    /// Get typed pointer
    pub fn get<T>(&self) -> *mut T { self.ptr as *mut T }

    /// Compute offset (in bytes) relative to base. Returns 0 if either is null.
    pub fn get_offset(&self, base: &BaseAddress) -> u32 {
        if self.ptr.is_null() || base.ptr.is_null() {
            return 0;
        }
        // Safety: pointer arithmetic on usize
        let a = self.ptr as usize;
        let b = base.ptr as usize;
        if a < b { 0 } else { (a - b) as u32 }
    }

    /// Check if this address points inside the given memory region [memory, memory+memory_size)
    pub fn in_range(&self, memory: *mut u8, memory_size: usize) -> bool {
        if self.ptr.is_null() || memory.is_null() { return false; }
        let start = memory as usize;
        let end = start.saturating_add(memory_size);
        let addr = self.ptr as usize;
        addr >= start && addr < end
    }

    /// Advance address by `n` bytes
    pub fn add_bytes(&self, n: usize) -> BaseAddress {
        if self.ptr.is_null() { return BaseAddress::null(); }
        BaseAddress { ptr: unsafe { self.ptr.add(n) } }
    }

    /// Subtract bytes
    pub fn sub_bytes(&self, n: usize) -> BaseAddress {
        if self.ptr.is_null() { return BaseAddress::null(); }
        BaseAddress { ptr: unsafe { self.ptr.sub(n) } }
    }

    /// Create a null BaseAddress
    pub fn null() -> BaseAddress { BaseAddress { ptr: std::ptr::null_mut() } }

    /// Create from a mutable pointer
    pub fn from_ptr(p: *mut u8) -> BaseAddress { BaseAddress { ptr: p } }

    /// Create from a const pointer
    pub fn from_const_ptr(p: *const u8) -> BaseAddress { BaseAddress { ptr: p as *mut u8 } }

    /// Whether pointer is null
    pub fn is_null(&self) -> bool { self.ptr.is_null() }
}


impl From<*mut u8> for BaseAddress {
    fn from(p: *mut u8) -> Self { BaseAddress::from_ptr(p) }
}
impl From<*const u8> for BaseAddress {
    fn from(p: *const u8) -> Self { BaseAddress::from_const_ptr(p) }
}

use std::ops::{Add, Sub};

impl Add<usize> for BaseAddress {
    type Output = BaseAddress;
    fn add(self, rhs: usize) -> Self::Output { self.add_bytes(rhs) }
}
impl Sub<usize> for BaseAddress {
    type Output = BaseAddress;
    fn sub(self, rhs: usize) -> Self::Output { self.sub_bytes(rhs) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_and_range_basic() {
        let mut buf = [0u8; 16];
        let base = BaseAddress::from_ptr(buf.as_mut_ptr());
        let a = base.add_bytes(4);
        assert_eq!(a.get_offset(&base), 4);
        assert!(a.in_range(buf.as_mut_ptr(), 16));
        let out = base.add_bytes(16);
        assert!(!out.in_range(buf.as_mut_ptr(), 16));
    }

    #[test]
    fn null_behaviour() {
        let n = BaseAddress::null();
        let b = BaseAddress::from_ptr(0 as *mut u8);
        assert_eq!(n.get_offset(&b), 0);
        assert!(!n.in_range(0 as *mut u8, 10));
    }
}

