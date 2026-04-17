/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for ActiveList (ported from original C++)

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ActiveList {
    list: Vec<usize>,
}

impl ActiveList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, id: usize) {
        if !self.list.contains(&id) {
            self.list.push(id);
        }
    }

    pub fn contains(&self, id: usize) -> bool {
        self.list.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.list.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::ActiveList;

    #[test]
    fn active_list_adds_unique_ids() {
        let mut list = ActiveList::new();
        list.add(1);
        list.add(2);
        list.add(1);

        assert_eq!(list.len(), 2);
        assert!(list.contains(1));
        assert!(list.contains(2));
    }
}
