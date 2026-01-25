/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Exception & error types (port of `GnaException.h`).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GnaError {
    #[error("GNA generic error: {0}")]
    Generic(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, GnaError>;
