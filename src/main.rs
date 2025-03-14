#![allow(clippy::cast_precision_loss)]

mod constants;
mod game;
mod graphic_utils;
mod level;
mod menu;
mod snake;
mod target;

use constants::{LEVEL_PATH, WINDOW_HEIGHT, WINDOW_WIDTH};
use game::{start_game, GameOutcome};
use level::{search_for_levels, Level};
use macroquad::prelude::*;
use macroquad::window;
use menu::GameMode;
use tracing::error;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Rusty Head Snake".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[derive(Clone)]
pub struct Context {
    font: Font,
}

#[macroquad::main(window_conf)]
async fn main() {
    let font: Font = load_ttf_font_from_bytes(include_bytes!("../assets/FiraSans-Black.ttf"))
        .expect("Failed to load text font");
    let cx = Context { font };

    loop {
        next_frame().await;
        let game_mode = menu::start(&cx).await;

        match game_mode {
            GameMode::EndlessGame => {
                start_game(&cx, &Level::default()).await;
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
                            println!("load level error");
                            error!(
                                ?err,
                                "Level {} is not valid and therefore skipped", level_name
                            );
                            continue;
                        }
                    };
                    loop {
                        match start_game(&cx, &level).await {
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
