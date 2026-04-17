/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/

/// Minimal Rust port of the GNA WindowsDriverInterface helper.
#[derive(Debug)]
pub struct WindowsDriverInterface {
    device_index: u32,
    opened: bool,
}

impl WindowsDriverInterface {
    pub fn new(device_index: u32) -> Self {
        Self {
            device_index,
            opened: false,
        }
    }

    pub fn open(&mut self) -> bool {
        self.opened = true;
        true
    }

    pub fn is_open(&self) -> bool {
        self.opened
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsDriverInterface;

    #[test]
    fn windows_driver_interface_open_marks_instance_as_open() {
        let mut driver = WindowsDriverInterface::new(0);
        assert!(!driver.is_open());

        assert!(driver.open());
        assert!(driver.is_open());
    }
}
