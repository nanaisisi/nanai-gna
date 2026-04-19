/// Re-export of the top-level `gna_api` module under `gna_lib::gna_api` to mirror
/// the original C++ layout (`gna-lib/gna-api`). This keeps a single implementation
/// in `crate::gna_api` while providing the expected module path used by ports.
pub use crate::gna_rs::gna_api::*;

// Local stubs mirroring original C++ files (progressive Rust port).
pub mod gna2_capability_impl;
pub mod gna2_common_impl;
pub mod gna2_device_impl;
pub mod gna2_impl;
pub mod gna2_inference_impl;
pub mod gna2_instrumentation_impl;
pub mod gna2_memory_impl;
pub mod gna2_model_export_impl;
pub mod gna2_model_impl;

// Optionally add local shims here in future if gna_lib-specific adaptations are needed.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reexports_are_accessible() {
        // A basic smoke test to ensure a known function is re-exported.
        // We don't call heavy logic here; just check symbol visibility.
        let _cfg = crate::gna_rs::gna_api::gna2_request_config_create();
        let _cfg2 = gna2_request_config_create();
        assert_eq!(
            std::mem::size_of::<Gna2RequestConfig>(),
            std::mem::size_of_val(&_cfg2)
        );
    }
}
