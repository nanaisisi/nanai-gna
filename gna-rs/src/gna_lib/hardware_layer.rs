/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Simplified Rust port of the GNA `HardwareLayer` helper.
#[derive(Debug, Default)]
pub struct HardwareLayer {
    configured: bool,
}

impl HardwareLayer {
    pub fn configure(&mut self) -> bool {
        self.configured = true;
        true
    }

    pub fn is_configured(&self) -> bool {
        self.configured
    }
}

#[cfg(test)]
mod tests {
    use super::HardwareLayer;

    #[test]
    fn hardware_layer_configure_sets_configured_flag() {
        let mut layer = HardwareLayer::default();
        assert!(!layer.is_configured());

        assert!(layer.configure());
        assert!(layer.is_configured());
    }
}
