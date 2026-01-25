//! Stub for HardwareModelNoMMU

#[allow(dead_code)]
pub struct HardwareModelNoMMU;

impl HardwareModelNoMMU {
    pub fn supports_no_mmu() -> bool { true }
}
