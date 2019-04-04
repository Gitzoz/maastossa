use noise::{Fbm, NoiseFn, Seedable};
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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
    generator: Fbm,
    seed: u32
}

pub trait LayerAccess {
    fn value_at(&self, x: f64, y: f64) -> f64;
}

impl Layer {
    pub fn new() -> Self {
        let seed = thread_rng().gen::<u32>();
        let generator = Fbm::new().set_seed(seed);
        Layer{
            generator,
            seed
        }
    }
}

impl LayerAccess for Layer {
    fn value_at(&self, x: f64, y: f64) -> f64 {
        (self.generator.get([x, y]) + 1.0) / 2.0
    }
}
