//! XNN kernel abstractions (port of `XnnKernel.h` / `XnnKernel.cpp`).

use super::kernel_arguments::KernelArguments;
use std::collections::BTreeMap;

/// Acceleration mode: CPU backend variant / hardware acceleration enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccelerationMode {
    Generic,
    SSE4,
    AVX1,
    AVX2,
    Hardware,
}

/// A kernel function type: takes `KernelArguments` and executes in place.
pub type KernelFn = fn(&KernelArguments);

/// Kernel map mapping acceleration mode to kernel implementation.
#[derive(Default)]
pub struct KernelMap {
    map: BTreeMap<AccelerationMode, KernelFn>,
}

impl KernelMap {
    pub fn new() -> Self { Self { map: BTreeMap::new() } }

    pub fn insert(&mut self, accel: AccelerationMode, f: KernelFn) { self.map.insert(accel, f); }

    pub fn get(&self, accel: AccelerationMode) -> Option<&KernelFn> { self.map.get(&accel) }

    /// Choose the best available kernel for requested acceleration mode (fallback to Generic)
    pub fn choose(&self, requested: AccelerationMode) -> Option<&KernelFn> {
        // try exact match first
        if let Some(f) = self.map.get(&requested) { return Some(f); }
        // fallback preference order
        for cand in [AccelerationMode::AVX2, AccelerationMode::AVX1, AccelerationMode::SSE4, AccelerationMode::Generic] {
            if let Some(f) = self.map.get(&cand) { return Some(f); }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_kernel(_args: &KernelArguments) { /* noop */ }

    #[test]
    fn kernel_map_basic() {
        let mut km = KernelMap::new();
        km.insert(AccelerationMode::Generic, dummy_kernel);
        km.insert(AccelerationMode::SSE4, dummy_kernel);
        assert!(km.get(AccelerationMode::SSE4).is_some());
        assert!(km.choose(AccelerationMode::AVX2).is_some()); // falls back to SSE4 or Generic
    }
}
