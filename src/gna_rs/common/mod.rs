/// Common utilities and types ported from `gna/common`.

pub mod address;
pub use address::BaseAddress;

pub mod profiler;
pub use profiler::Profiler;

pub mod macros;
pub use macros::*;

pub mod gna_exception;
pub use gna_exception::{GnaError, Result};

pub mod gna_drv_api;
pub use gna_drv_api::{GnaDriver, SoftwareDriver};

pub mod gna_drm;
pub use gna_drm::*;

pub mod gna_h_wrapper;

pub mod resource;
pub use resource::*;

// note: BufferMap belongs to `gna-lib` in the original source; do not export it from `common`.
