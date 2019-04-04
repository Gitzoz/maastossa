use noise::{Seedable, NoiseFn, Fbm};
use crate::layer::{LayerAccess, Layer};
use crate::render::Render;
use noise::utils::{PlaneMapBuilder, NoiseMapBuilder};

mod layer;
mod render;
mod manager;
mod terrain;

fn main() {

    let terrain = terrain::Terrain::new(10, 10);

    terrain.render()

}
