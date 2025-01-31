extern crate euclid;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
mod constants;
mod game_loop;
mod snake;
mod target;

use constants::{GAME_FIELD_HEIGHT, GAME_FIELD_WIDTH, SCALING_FACTOR};
use game_loop::{game_loop, App};
use glutin_window::OpenGL;
use opengl_graphics::GlGraphics;
use piston::WindowSettings;
use snake::Snake;
use target::Target;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window = WindowSettings::new(
        "spinning-square",
        [
            GAME_FIELD_WIDTH * SCALING_FACTOR,
            GAME_FIELD_HEIGHT * SCALING_FACTOR,
        ],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: Snake::new(),
        target: Target::new(),
    };

    game_loop(&mut app, &mut window);
}
