use noise::{Seedable, NoiseFn, Fbm};
use crate::layer::{LayerAccess, Layer};
use crate::render::Render;
use noise::utils::{PlaneMapBuilder, NoiseMapBuilder};

mod layer;
mod render;

fn main() {
    let fbm = Fbm::new();

    let seeded_fbm = fbm.set_seed(123);

    let base_layer = layer::Layer::new();

    let renderer = render::Renderer::new(10, 10, base_layer);
    renderer.render()

}
