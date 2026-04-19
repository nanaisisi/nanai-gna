/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
// Skeleton for `gna2-device-api.h`.

/// Device version placeholder
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Gna2DeviceVersion(pub u32);

/// The canonical Gna2Status type is defined in `common_api`.
pub use crate::gna_rs::gna_api::common_api::Gna2Status;

use std::sync::OnceLock;

#[cfg(unix)]
use crate::gna_rs::common::gna_drv_api::LinuxGnaDriver;
use crate::gna_rs::common::{SoftwareDriver, gna_exception::Result as GnaResult};
use crate::gna_rs::gna_lib::driver_interface::DriverInterface;

static DEVICE_DRIVER: OnceLock<
    Box<dyn crate::gna_rs::common::gna_drv_api::GnaDriver + Send + Sync>,
> = OnceLock::new();

fn device_driver() -> &'static dyn crate::gna_rs::common::gna_drv_api::GnaDriver {
    DEVICE_DRIVER
        .get_or_init(|| {
            #[cfg(unix)]
            {
                let linux_driver = LinuxGnaDriver::new();
                if linux_driver.get_device_count().unwrap_or(0) > 0 {
                    return Box::new(linux_driver);
                }
            }

            Box::new(SoftwareDriver::default())
        })
        .as_ref()
}

/// Get the number of available GNA devices.
pub fn gna2_device_get_count() -> GnaResult<u32> {
    device_driver().get_device_count()
}

/// Open a GNA device by index.
pub fn gna2_device_open(device_index: u32) -> GnaResult<()> {
    device_driver().device_open(device_index)
}

/// Close a GNA device by index.
pub fn gna2_device_close(device_index: u32) -> GnaResult<()> {
    device_driver().device_close(device_index)
}

/// Query the version of a GNA device.
pub fn gna2_device_get_version(device_index: u32) -> GnaResult<Gna2DeviceVersion> {
    let device_count = gna2_device_get_count()?;
    if device_index >= device_count {
        return Err(crate::gna_rs::common::gna_exception::GnaError::NotFound(
            format!("device index {} is out of range", device_index),
        ));
    }

    Ok(DriverInterface::query(device_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gna2_device_api_get_count_returns_non_zero() {
        let count = gna2_device_get_count().expect("device count should be available");
        assert!(count >= 1);
    }

    #[test]
    fn gna2_device_api_get_version_returns_default_version_for_first_device() {
        let version = gna2_device_get_version(0).expect("version should be available");
        assert_eq!(version, Gna2DeviceVersion(0x30));
    }

    #[test]
    fn gna2_device_api_open_close_device_succeeds() {
        assert!(gna2_device_open(0).is_ok());
        assert!(gna2_device_close(0).is_ok());
    }

    #[test]
    fn gna2_device_api_get_version_rejects_invalid_index() {
        let count = gna2_device_get_count().unwrap_or(1);
        let result = gna2_device_get_version(count);
        assert!(result.is_err());
    }
}
