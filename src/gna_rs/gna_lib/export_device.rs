/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
use crate::gna_rs::gna_api::model_api::Gna2Model;
use crate::gna_rs::gna_lib::device::Device;

/// Minimal Rust port of the GNA `ExportDevice` helper.
#[derive(Debug)]
pub struct ExportDevice {
    pub(crate) device: Device,
    target_device_version: Gna2DeviceVersion,
    model_id: Option<u32>,
}

impl ExportDevice {
    pub fn new(target_device_version: Gna2DeviceVersion) -> Self {
        Self {
            device: Device::new(target_device_version),
            target_device_version,
            model_id: None,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn load_model(&mut self, model: &Gna2Model) -> bool {
        let model_id = self.device.load_model(model);
        self.model_id = Some(model_id);
        true
    }

    pub fn export(&self) -> bool {
        self.model_id.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::ExportDevice;
    use crate::gna_rs::gna_api::device_api::Gna2DeviceVersion;
    use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn export_device_loads_and_exports_model() {
        let mut device = ExportDevice::new(Gna2DeviceVersion(0x30));
        assert!(!device.export());

        let mut model = Gna2Model::new();
        model.add_operation(Gna2Operation::default());

        assert!(device.load_model(&model));
        assert!(device.export());
    }
}
