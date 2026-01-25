//! Skeleton for `gna2-capability-api.h` (capabilities / hardware features)

bitflags::bitflags! {
    /// Hardware capabilities flags (partial)
    pub struct HardwareCapabilities: u32 {
        const CNN1D = 0x1;
        const LEGACY_GMM = 0x2;
    }
}
