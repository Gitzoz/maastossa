use noise::{Fbm, NoiseFn};

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
        self.generator.get([x, y])
    }
}
