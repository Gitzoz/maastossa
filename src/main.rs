use glutin_window::{GlutinWindow, OpenGL};
use opengl_graphics::GlGraphics;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use crate::render::{Render, Renderer};

mod layer;
mod render;
mod manager;
mod terrain;

fn main() {
    let terrain = terrain::Terrain::new(50, 50);

    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "Terrain",
        [terrain.width as u32 * 10, terrain.height as u32 * 10],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Renderer::new(GlGraphics::new(opengl), terrain);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
