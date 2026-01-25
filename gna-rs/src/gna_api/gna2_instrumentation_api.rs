//! Skeleton for `gna2-instrumentation-api.h`.

/// Instrumentation point placeholders (partial)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2InstrumentationPoint {
    LibPreprocessing = 0,
    LibSubmission = 1,
    LibProcessing = 2,
    LibExecution = 3,
    LibDeviceRequestReady = 4,
    LibDeviceRequestSent = 5,
    LibDeviceRequestCompleted = 6,
    LibCompletion = 7,
    LibReceived = 8,
    DrvPreprocessing = 9,
    HwTotalCycles = 13,
    HwStallCycles = 14,
}

/// Units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gna2InstrumentationUnit { Microseconds, Milliseconds, Cycles }
