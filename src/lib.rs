//! Rust bindings for the Intel® Gaussian & Neural Accelerator (GNA) 3.0 C API.
//!
//! The bindings are generated at build time from the upstream headers located in
//! `gna/src/gna-api/gna2-api.h`. All symbols from the C interface are exposed
//! under the [`raw`] module. No linking is performed automatically; you are
//! expected to provide the appropriate GNA runtime library (e.g. `gna.dll`/
//! `libgna.so`) when building a final binary that calls these functions.

/// Low-level, automatically generated FFI bindings to the C API.
///
/// The module intentionally suppresses common lint warnings because it mirrors
/// the original C naming conventions.
pub mod raw {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        clippy::all
    )]

    include!(concat!(env!("OUT_DIR"), "/gna_bindings.rs"));
}

/// Re-export the raw bindings at the crate root for convenience.
pub use raw::*;

pub mod instrumentation;

#[cfg(test)]
mod tests {
    use super::raw;

    #[test]
    fn bindings_basic_type_is_available() {
        let _ = std::mem::size_of::<raw::Gna2Status>();
    }
}
