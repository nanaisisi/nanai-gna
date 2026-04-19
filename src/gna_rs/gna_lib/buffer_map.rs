use crate::gna_rs::common::BaseAddress;
use crate::gna_rs::gna_api::types::{INPUT_OPERAND_INDEX, OUTPUT_OPERAND_INDEX};
/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// `BufferMap` lives in `gna-lib` in the original codebase. This mirrors the
/// original small API: mapping from operand (u32) to `BaseAddress`.
use std::collections::BTreeMap;

/// Simple `BufferMap` compatible with original structure.
#[derive(Debug, Default, Clone)]
pub struct BufferMap {
    inner: BTreeMap<u32, BaseAddress>,
}

impl BufferMap {
    /// Default constructor
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Construct with input / output addresses pre-populated.
    pub fn with_io(input: BaseAddress, output: BaseAddress) -> Self {
        let mut bm = Self::new();
        if !input.is_null() {
            bm.emplace(INPUT_OPERAND_INDEX, input);
        }
        if !output.is_null() {
            bm.emplace(OUTPUT_OPERAND_INDEX, output);
        }
        bm
    }

    /// Insert or replace mapping
    pub fn insert(&mut self, operand: u32, addr: BaseAddress) {
        self.inner.insert(operand, addr);
    }

    /// Emplace: insert only if key not present. Returns true if inserted.
    pub fn emplace(&mut self, operand: u32, addr: BaseAddress) -> bool {
        use std::collections::btree_map::Entry;
        match self.inner.entry(operand) {
            Entry::Vacant(e) => {
                e.insert(addr);
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    /// Find an entry
    pub fn find(&self, operand: u32) -> Option<(&u32, &BaseAddress)> {
        self.inner.get_key_value(&operand)
    }

    /// Count occurrences (0 or 1)
    pub fn count(&self, operand: u32) -> usize {
        if self.inner.contains_key(&operand) {
            1
        } else {
            0
        }
    }

    /// Remove mapping
    pub fn erase(&mut self, operand: &u32) -> Option<BaseAddress> {
        self.inner.remove(operand)
    }

    /// Get a copy of value if present
    pub fn get(&self, operand: u32) -> Option<BaseAddress> {
        self.inner.get(&operand).cloned()
    }

    /// Iterators
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &BaseAddress)> {
        self.inner.iter()
    }
    pub fn keys(&self) -> impl Iterator<Item = &u32> {
        self.inner.keys()
    }

    /// Number of elements
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

use std::ops::{Index, IndexMut};
impl Index<u32> for BufferMap {
    type Output = BaseAddress;
    fn index(&self, index: u32) -> &Self::Output {
        self.inner
            .get(&index)
            .expect("BufferMap index out of range")
    }
}
impl IndexMut<u32> for BufferMap {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        self.inner.entry(index).or_insert(BaseAddress::null())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn with_io_populates() {
        let in_addr = BaseAddress::from_ptr(0x1000usize as *mut u8);
        let out_addr = BaseAddress::from_ptr(0x2000usize as *mut u8);
        let bm = BufferMap::with_io(in_addr, out_addr);
        assert_eq!(bm.get(INPUT_OPERAND_INDEX), Some(in_addr));
        assert_eq!(bm.get(OUTPUT_OPERAND_INDEX), Some(out_addr));
    }

    #[test]
    fn emplace_behaviour() {
        let mut bm = BufferMap::new();
        let a = BaseAddress::from_ptr(0x100usize as *mut u8);
        assert!(bm.emplace(10, a));
        assert!(!bm.emplace(10, BaseAddress::from_ptr(0x200usize as *mut u8)));
        assert_eq!(bm.get(10), Some(a));
    }

    #[test]
    fn index_assignment() {
        let mut bm = BufferMap::new();
        bm[5u32] = BaseAddress::from_ptr(0x300usize as *mut u8);
        assert_eq!(
            bm.get(5),
            Some(BaseAddress::from_ptr(0x300usize as *mut u8))
        );
    }
}
