/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Skeleton for `CompiledModel` from `gna-lib`.

/// Represents a compiled model (simplified placeholder).
#[derive(Debug, Clone)]
pub struct CompiledModel {
    id: u32,
    pub model: crate::gna_rs::gna_api::model_api::Gna2Model,
}

static NEXT_COMPILED_MODEL_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

impl CompiledModel {
    pub fn new(model: crate::gna_rs::gna_api::model_api::Gna2Model) -> Self {
        let id = NEXT_COMPILED_MODEL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self { id, model }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn operation_count(&self) -> u32 {
        self.model.operation_count()
    }

    pub fn get_operation(
        &self,
        operation_index: usize,
    ) -> Option<&crate::gna_rs::gna_api::model_api::Gna2Operation> {
        self.model.operations.get(operation_index)
    }

    pub fn get_operations(&self) -> &[crate::gna_rs::gna_api::model_api::Gna2Operation] {
        &self.model.operations
    }
}

#[cfg(test)]
mod tests {
    use super::CompiledModel;
    use crate::gna_rs::gna_api::model_api::{Gna2Model, Gna2Operation};
    use crate::gna_rs::gna_api::types::OperationType;

    #[test]
    fn compiled_model_tracks_id_and_operations() {
        let mut model = Gna2Model::new();
        let op = Gna2Operation {
            op_type: OperationType::Copy,
            number_of_operands: 0,
            number_of_parameters: 0,
            operands: vec![],
            parameters: vec![],
        };
        model.add_operation(op.clone());
        let compiled = CompiledModel::new(model);

        assert!(compiled.id() > 0);
        assert_eq!(compiled.operation_count(), 1);
        assert_eq!(compiled.get_operation(0), Some(&op));
        assert_eq!(compiled.get_operations().len(), 1);
    }
}
