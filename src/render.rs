extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::terrain::{NatureType, Terrain};

pub struct Renderer {
    gl: GlGraphics,
    terrain: Terrain,
}

impl Renderer {
    pub fn new(gl: GlGraphics, terrain: Terrain) -> Self {
        Renderer {
            gl,
            terrain,
        }
    }
}

pub trait Render {
    fn render(&mut self, _args: &RenderArgs) -> ();
    fn update(&mut self, _args: &UpdateArgs) -> ();
}

impl Render for Renderer {
    fn render(&mut self, _args: &RenderArgs) -> () {
        use graphics::*;
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BROWN: [f32; 4] = [0.40, 0.21, 0.05, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 10.0);

        let nature: Vec<Vec<NatureType>> = self.terrain
            .composition
            .iter()
            .map(|inner| {
                let as_nature_type = inner.iter().map(|nature| nature.get_type()).collect();
                as_nature_type
            })
            .collect();

        self.gl.draw(_args.viewport(), |c, gl| {
            clear(RED, gl);
            for (x, y_nature) in nature.iter().enumerate() {
                for (y, nature) in y_nature.iter().enumerate() {
                    let transform = c.transform.trans(x as f64 * 10.0, y as f64 * 10.0);

                    match nature {
                        NatureType::WATER => rectangle(BLUE, square, transform, gl),
                        NatureType::LAND => rectangle(BROWN, square, transform, gl),
                        NatureType::FOREST => rectangle(GREEN, square, transform, gl)
                    }
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) -> () {}
}