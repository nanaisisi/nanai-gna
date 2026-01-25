//! Stub for Shape

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Shape(pub Vec<usize>);

impl Shape {
    pub fn new() -> Self { Self(Vec::new()) }
}
