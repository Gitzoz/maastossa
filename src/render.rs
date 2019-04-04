use crate::layer::{LayerAccess, Layer};
use crate::terrain::Terrain;

pub trait Render {
    fn render(&self) -> ();
}

impl Render for Terrain {
    fn render(&self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                println!("{:?}", self.composition[x][y]);
            }
        }
    }
}