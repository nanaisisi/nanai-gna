/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `CompiledModel` from `gna-lib`.

/// Represents a compiled model (simplified placeholder).
#[derive(Debug, Clone)]
pub struct CompiledModel {
    id: u32,
    pub model: crate::gna_api::model_api::Gna2Model,
}

static NEXT_COMPILED_MODEL_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

impl CompiledModel {
    pub fn new(model: crate::gna_api::model_api::Gna2Model) -> Self {
        let id = NEXT_COMPILED_MODEL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self { id, model }
    }

    pub fn id(&self) -> u32 { self.id }
}
