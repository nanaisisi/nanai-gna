//! Skeleton for `gna2-device-api.h`.

/// Device version placeholder
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Gna2DeviceVersion(pub u32);

/// The canonical Gna2Status type is defined in `common_api`.
pub use crate::gna_api::common_api::Gna2Status;

