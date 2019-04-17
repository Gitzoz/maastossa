use noise::{NoiseFn, SuperSimplex, Seedable};
use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LayerType {
    Height,
    Water,
    Forest,
}

impl LayerType {
    pub fn all() -> Vec<LayerType> {
        vec![LayerType::Height, LayerType::Water, LayerType::Forest]
    }
}

pub struct Layer {
    generator: SuperSimplex,
    seed: u32,
}

pub trait LayerAccess {
    fn value_at(&self, x: f64, y: f64) -> f64;
}

impl Layer {
    pub fn new() -> Self {
        let seed = thread_rng().gen::<u32>();
        let generator = SuperSimplex::new().set_seed(seed);
        Layer {
            generator,
            seed,
        }
    }
}

impl LayerAccess for Layer {
    fn value_at(&self, x: f64, y: f64) -> f64 {
        (self.generator.get([x, y]) + 1.0) / 2.0
    }
}
