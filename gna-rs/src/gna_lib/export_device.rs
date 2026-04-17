/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_api::device_api::Gna2DeviceVersion;

/// Minimal Rust port of the GNA `ExportDevice` helper.
#[derive(Debug)]
pub struct ExportDevice {
    target_device_version: Gna2DeviceVersion,
    model_loaded: bool,
}

impl ExportDevice {
    pub fn new(target_device_version: Gna2DeviceVersion) -> Self {
        Self {
            target_device_version,
            model_loaded: false,
        }
    }

    pub fn load_model(&mut self) -> bool {
        self.model_loaded = true;
        true
    }

    pub fn export(&self) -> bool {
        self.model_loaded
    }
}

#[cfg(test)]
mod tests {
    use super::ExportDevice;
    use crate::gna_api::device_api::Gna2DeviceVersion;

    #[test]
    fn export_device_loads_and_exports_model() {
        let mut device = ExportDevice::new(Gna2DeviceVersion(0x30));
        assert!(!device.export());

        assert!(device.load_model());
        assert!(device.export());
    }
}
