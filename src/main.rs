extern crate euclid;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate tracing;
mod constants;
mod game;
mod graphic_utils;
mod level;
mod menu;
mod snake;
mod target;
mod text;

use constants::{LEVEL_PATH, WINDOW_HEIGHT, WINDOW_WIDTH};
use game::{start_game, GameOutcome};
use glutin_window::OpenGL;
use level::{search_for_levels, Level};
use menu::GameMode;
use opengl_graphics::GlGraphics;
use piston::WindowSettings;
use tracing::error;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window = WindowSettings::new("Rusty Head Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .build()
        .unwrap();
    let mut opengl_backend = GlGraphics::new(opengl);

    loop {
        let game_mode = menu::start(&mut window, &mut opengl_backend);

        match game_mode {
            GameMode::EndlessGame => {
                start_game(&mut window, &mut opengl_backend, &Level::default());
            }
            GameMode::Levels => {
                let level_names = match search_for_levels(LEVEL_PATH) {
                    Ok(paths) => paths,
                    Err(err) => {
                        error!(?err, "No levels found");
                        continue;
                    }
                };
                'level_loop: for level_name in &level_names {
                    let level = match Level::load_level(LEVEL_PATH, level_name) {
                        Ok(level) => level,
                        Err(err) => {
                            error!(
                                ?err,
                                "Level {} is not valid and therefore skipped", level_name
                            );
                            continue;
                        }
                    };
                    loop {
                        match start_game(&mut window, &mut opengl_backend, &level) {
                            GameOutcome::Win => break,
                            GameOutcome::Exit => break 'level_loop,
                            GameOutcome::Lose => {}
                        }
                    }
                }
            }
            GameMode::Exit => break,
        }
    }
}
