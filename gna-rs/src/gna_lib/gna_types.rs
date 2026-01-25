/**
 @copyright Copyright (C) 2020-2022 Intel Corporation
 SPDX-License-Identifier: LGPL-2.1-or-later
*/
/// Basic GNA types (minimal stubs)

#[allow(dead_code)]
pub type GnaAddress = usize;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum GnaDataType {
    I16,
    I8,
}
