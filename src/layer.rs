use noise::{Fbm, NoiseFn};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LayerType {
    Height,
    Water
}

impl LayerType {
    pub fn all() -> Vec<LayerType> {
        vec![LayerType::Height, LayerType::Water]
    }
}

pub struct Layer {
    generator: Fbm
}

pub trait LayerAccess {
    fn value_at(&self, x: f64, y: f64) -> f64;
}

impl Layer {
    pub fn new() -> Self {
        Layer{
            generator: Fbm::new()
        }
    }

    pub fn new_with_generator(generator: Fbm) -> Self {
        Layer{
            generator
        }
    }
}

impl LayerAccess for Layer {
    fn value_at(&self, x: f64, y: f64) -> f64 {
        (self.generator.get([x, y]) + 1) / 2
    }
}
