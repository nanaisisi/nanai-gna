/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Stub for CopyLayer (ported from original C++)

#[derive(Debug, Default, Clone, Copy)]
pub struct CopyLayer;

impl CopyLayer {
    pub fn copy<T: Copy>(&self, source: &[T], destination: &mut [T]) {
        assert_eq!(
            source.len(),
            destination.len(),
            "source and destination must have the same length"
        );
        destination.copy_from_slice(source);
    }
}

#[cfg(test)]
mod tests {
    use super::CopyLayer;

    #[test]
    fn copy_layer_copies_data_into_destination() {
        let copier = CopyLayer::default();
        let source = [1i16, 2, 3, 4];
        let mut destination = [0i16; 4];

        copier.copy(&source, &mut destination);

        assert_eq!(destination, source);
    }
}
