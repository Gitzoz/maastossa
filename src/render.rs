use crate::layer::{LayerAccess, Layer};

pub struct Renderer {
    width: usize,
    height: usize,
    layer: Layer
}

impl Renderer {
    pub fn new(width: usize, height: usize, layer: Layer) -> Self {
        Renderer{
            width,
            height,
            layer
        }
    }
}

pub trait Render {
    fn render(&self) -> ();
}

impl Render for Renderer {
    fn render(&self) -> () {
        let mut field = vec![vec![0.0; self.width]; self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                field[x][y] = self.layer.value_at(x as f64, y as f64)
            }
        }

        println!("{:?}", field);
    }
}