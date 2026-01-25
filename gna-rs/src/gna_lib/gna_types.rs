//! Basic GNA types (minimal stubs)

#[allow(dead_code)]
pub type GnaAddress = usize;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum GnaDataType {
    I16,
    I8,
}
