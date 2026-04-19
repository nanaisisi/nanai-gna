/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Acceleration detector and kernel registry (port of `AccelerationDetector`).

use super::xnn_kernel::{KernelMap, AccelerationMode, KernelFn};
use super::kernel_arguments::KernelArguments;

/// Runtime detector that provides kernel maps for specific kernel types.
/// In the original code this detects CPU features and hardware capabilities.
pub struct AccelerationDetector;

impl AccelerationDetector {
    /// Return a kernel map for the requested kernel 'type' represented by a string key.
    /// In a full port this would inspect CPU features and provide optimized kernels.
    pub fn get_kernel_map(_name: &str) -> KernelMap {
        // For now return an empty map; individual kernel ports will populate this map.
        KernelMap::new()
    }

    /// Example helper for registering a kernel at runtime. Not part of original API but
    /// useful for tests/dummy registrations.
    pub fn register_kernel(map: &mut KernelMap, accel: AccelerationMode, f: KernelFn) {
        map.insert(accel, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::AccelerationMode;
    use super::KernelArguments;

    fn example_kernel(_a: &KernelArguments) {}

    #[test]
    fn detector_returns_map() {
        let mut m = AccelerationDetector::get_kernel_map("transpose");
        AccelerationDetector::register_kernel(&mut m, AccelerationMode::Generic, example_kernel);
        assert!(m.choose(AccelerationMode::Generic).is_some());
    }
}
