/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::gna_rs::common::gna_exception::GnaError;
use crate::gna_rs::gna_api::common_api::Gna2Status;
use crate::gna_rs::gna_lib::logger::Logger;

/// Rust port of the GNA ApiWrapper helper.
#[allow(dead_code)]
pub struct ApiWrapper;

impl ApiWrapper {
    /// Execute a command safely, log any error, and return a fallback value.
    pub fn execute_safely<T, F>(command: F, fallback: T) -> T
    where
        F: FnOnce() -> Result<T, GnaError> + std::panic::UnwindSafe,
    {
        match catch_unwind(AssertUnwindSafe(command)) {
            Ok(Ok(value)) => value,
            Ok(Err(err)) => {
                Self::log_exception(&err);
                fallback
            }
            Err(payload) => {
                let message = Self::panic_message(payload);
                Self::log_exception(&GnaError::Generic(message));
                fallback
            }
        }
    }

    /// Execute a GNA status-producing command safely.
    pub fn execute_safely_status<F>(command: F) -> Gna2Status
    where
        F: FnOnce() -> Result<Gna2Status, GnaError> + std::panic::UnwindSafe,
    {
        const UNKNOWN_ERROR: Gna2Status = -3;
        Self::execute_safely(command, UNKNOWN_ERROR)
    }

    /// Handle and log a panic, returning an unknown error status.
    pub fn handle_and_log_exceptions() -> Gna2Status {
        const UNKNOWN_ERROR: Gna2Status = -3;
        if let Err(payload) = catch_unwind(AssertUnwindSafe(|| panic!())) {
            let message = Self::panic_message(payload);
            Self::log_exception(&GnaError::Generic(message));
        }
        UNKNOWN_ERROR
    }

    fn log_exception(error: &dyn std::fmt::Display) {
        let logger = Logger;
        logger.log(&format!("ApiWrapper exception: {}", error));
    }

    fn panic_message(payload: Box<dyn Any + Send>) -> String {
        if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_safely_returns_result_value() {
        let value = ApiWrapper::execute_safely(|| Ok(42), 0);
        assert_eq!(value, 42);
    }

    #[test]
    fn execute_safely_returns_fallback_on_error() {
        let value = ApiWrapper::execute_safely(|| Err(GnaError::Generic("fail".into())), 99);
        assert_eq!(value, 99);
    }

    #[test]
    fn execute_safely_status_returns_unknown_on_error() {
        let status = ApiWrapper::execute_safely_status(|| Err(GnaError::Generic("fail".into())));
        assert_eq!(status, -3);
    }

    #[test]
    fn handle_and_log_exceptions_returns_unknown_error() {
        let status = ApiWrapper::handle_and_log_exceptions();
        assert_eq!(status, -3);
    }
}
