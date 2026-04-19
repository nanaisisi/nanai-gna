// Re-export the grouped gna2 modules under `crate::gna_api`

// Directly expose the raw gna2_ modules (so wrappers can refer to them)
pub mod gna2_common_api;
pub mod gna2_model_api;
pub mod gna2_inference_api;
pub mod gna2_memory_api;
pub mod gna2_device_api;
pub mod gna2_capability_api;
pub mod gna2_instrumentation_api;
pub mod gna2_model_export_api;
pub mod gna2_suecreek_header;
pub mod gna2_types;

// Note: `gna2_mod` holds per-header modules (types, inference_api, memory_api, ...).

// Provide lightweight compatibility re-exports so other modules can use `crate::gna_api::types` etc.
pub mod types { pub use super::gna2_types::*; }
pub mod common_api { pub use super::gna2_common_api::*; }
pub mod model_api { pub use super::gna2_model_api::*; }
pub mod inference_api { pub use super::gna2_inference_api::*; }
pub mod memory_api { pub use super::gna2_memory_api::*; }
pub mod device_api { pub use super::gna2_device_api::*; }
pub mod capability_api { pub use super::gna2_capability_api::*; }
pub mod instrumentation_api { pub use super::gna2_instrumentation_api::*; }
pub mod model_export_api { pub use super::gna2_model_export_api::*; }
pub mod suecreek_header { pub use super::gna2_suecreek_header::*; }

// Re-export common items at crate::gna_api root for convenience (matches previous expectations)
pub use inference_api::*;
pub use types::*;