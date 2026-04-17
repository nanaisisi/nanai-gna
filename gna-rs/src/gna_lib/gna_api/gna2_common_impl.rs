#![allow(dead_code)]
/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Rust port for original: gna/src/gna-lib/gna-api/gna2-common-impl.h
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::gna_api::common_api::Gna2Status;
use crate::gna_api::device_api::Gna2DeviceVersion;

/// Constant used to indicate a disabled value in the API.
pub const GNA2_DISABLED_U32: u32 = u32::MAX;

/// Size of memory alignment for data tensors.
pub const GNA_MEM_ALIGN: u32 = 64;

/// Default software emulation device version.
pub const DEFAULT_DEVICE_VERSION: Gna2DeviceVersion = Gna2DeviceVersion(0x30);

/// Convert an integer value into a device version wrapper.
pub fn gna2_device_version_from_int(value: u32) -> Gna2DeviceVersion {
    Gna2DeviceVersion(value)
}

lazy_static! {
    static ref STATUS_STRING_MAP: HashMap<Gna2Status, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, "Gna2StatusSuccess");
        map.insert(1, "Gna2StatusWarningDeviceBusy");
        map.insert(2, "Gna2StatusWarningArithmeticSaturation");
        map.insert(3, "Gna2StatusModelErrorUnavailable");
        map.insert(-3, "Gna2StatusUnknownError");
        map.insert(-4, "Gna2StatusNotImplemented");
        map.insert(-5, "Gna2StatusIdentifierInvalid");
        map.insert(-6, "Gna2StatusNullArgumentNotAllowed");
        map.insert(-7, "Gna2StatusNullArgumentRequired");
        map.insert(-8, "Gna2StatusResourceAllocationError");
        map.insert(-9, "Gna2StatusDeviceNotAvailable");
        map.insert(-10, "Gna2StatusDeviceNumberOfThreadsInvalid");
        map.insert(-11, "Gna2StatusDeviceVersionInvalid");
        map.insert(-12, "Gna2StatusDeviceQueueError");
        map.insert(-13, "Gna2StatusDeviceIngoingCommunicationError");
        map
    };
}

pub struct StatusHelper;

impl StatusHelper {
    pub fn get_string_map() -> &'static HashMap<Gna2Status, &'static str> {
        &STATUS_STRING_MAP
    }

    pub fn to_string(status_in: Gna2Status) -> String {
        Self::get_string_map()
            .get(&status_in)
            .copied()
            .unwrap_or("Gna2StatusUnknown")
            .to_string()
    }
}

/// Returns the mapped value if present, otherwise returns the provided default.
pub fn get_mapped_or_default<K, V>(key: &K, default_value: V, map: &HashMap<K, V>) -> V
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    map.get(key).cloned().unwrap_or(default_value)
}

/// Returns `true` when the map contains the given key.
pub fn contains<K, V>(container: &HashMap<K, V>, key: &K) -> bool
where
    K: Eq + std::hash::Hash,
{
    container.contains_key(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn default_device_version_is_3_0() {
        assert_eq!(DEFAULT_DEVICE_VERSION.0, 0x30);
    }

    #[test]
    fn device_version_from_int_wraps_value() {
        let version = gna2_device_version_from_int(0x20);
        assert_eq!(version.0, 0x20);
    }

    #[test]
    fn status_helper_returns_known_status_string() {
        assert_eq!(StatusHelper::to_string(0), "Gna2StatusSuccess");
        assert_eq!(
            StatusHelper::to_string(-11),
            "Gna2StatusDeviceVersionInvalid"
        );
    }

    #[test]
    fn status_helper_returns_unknown_for_unmapped_status() {
        assert_eq!(StatusHelper::to_string(999), "Gna2StatusUnknown");
    }

    #[test]
    fn get_mapped_or_default_returns_value_or_default() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        assert_eq!(get_mapped_or_default(&1, "default", &map), "one");
        assert_eq!(get_mapped_or_default(&2, "default", &map), "default");
    }

    #[test]
    fn contains_returns_true_if_key_present() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        assert!(contains(&map, &1));
        assert!(!contains(&map, &2));
    }
}
