use crate::gna_lib::parameter_limits::AlignLimits;
/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::sync::Arc;

/// Simplified Rust port of the GNA Validator helper.
#[derive(Clone)]
pub struct BaseValidator {
    pub generation: u32,
    buffer_validator: Arc<dyn Fn(*const u8, usize, u32) -> bool + Send + Sync>,
}

impl std::fmt::Debug for BaseValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseValidator")
            .field("generation", &self.generation)
            .field("buffer_validator", &"<functor>")
            .finish()
    }
}

impl BaseValidator {
    pub fn new(
        generation: u32,
        buffer_validator: Arc<dyn Fn(*const u8, usize, u32) -> bool + Send + Sync>,
    ) -> Self {
        Self {
            generation,
            buffer_validator,
        }
    }

    pub fn validate_buffer_if_set(
        &self,
        buffer: *const u8,
        size: usize,
        align_limits: AlignLimits,
    ) -> bool {
        if buffer.is_null() {
            true
        } else {
            self.validate_buffer(buffer, size, align_limits.value as u32)
        }
    }

    fn validate_buffer(&self, buffer: *const u8, size: usize, alignment: u32) -> bool {
        (self.buffer_validator)(buffer, size, alignment)
    }
}

#[derive(Clone)]
pub struct LayerValidator {
    pub base: BaseValidator,
    pub operation: u32,
}

impl std::fmt::Debug for LayerValidator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerValidator")
            .field("base", &self.base)
            .field("operation", &self.operation)
            .finish()
    }
}

impl LayerValidator {
    pub fn new(base: &BaseValidator, operation: u32) -> Self {
        Self {
            base: base.clone(),
            operation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Validator {
    layer_validator: LayerValidator,
    pub capabilities: Option<()>,
    pub order: u32,
    pub is_buffer_optional: bool,
}

impl Validator {
    pub fn new(
        validator: &LayerValidator,
        capabilities: Option<()>,
        is_buffer_optional: bool,
    ) -> Self {
        Self {
            layer_validator: validator.clone(),
            capabilities,
            order: 0,
            is_buffer_optional,
        }
    }

    pub fn validate_buffer(&self, buffer: *const u8, size: usize, alignment: u32) -> bool {
        if self.is_buffer_optional {
            self.layer_validator.base.validate_buffer_if_set(
                buffer,
                size,
                AlignLimits::new(alignment, 0),
            )
        } else {
            self.layer_validator
                .base
                .validate_buffer(buffer, size, alignment)
        }
    }

    pub fn validate(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_buffer_validator(buffer: *const u8, size: usize, alignment: u32) -> bool {
        if buffer.is_null() {
            false
        } else {
            let address = buffer as usize;
            address % (alignment as usize) == 0 && size > 0
        }
    }

    #[test]
    fn base_validator_validate_buffer_if_set_passes_for_null_buffer() {
        let validator = BaseValidator::new(0, Arc::new(default_buffer_validator));
        let align_limits = AlignLimits::new(64, 0);
        assert!(validator.validate_buffer_if_set(std::ptr::null(), 0, align_limits));
    }

    #[test]
    fn base_validator_validate_buffer_fails_for_misaligned_pointer() {
        let validator = BaseValidator::new(0, Arc::new(default_buffer_validator));
        let data = [0u8; 8];
        let ptr = data.as_ptr();
        assert!(!validator.validate_buffer_if_set(ptr, data.len(), AlignLimits::new(16, 0)));
    }

    #[test]
    fn validator_validate_buffer_optional_accepts_null_buffer() {
        let base = BaseValidator::new(0, Arc::new(default_buffer_validator));
        let layer = LayerValidator::new(&base, 0);
        let validator = Validator::new(&layer, None, true);
        assert!(validator.validate_buffer(std::ptr::null(), 0, 16));
    }
}
