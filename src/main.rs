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
use graphic_utils::render_error_message;
use level::{base_levels, search_for_custom_levels, Level};
use macroquad::prelude::*;
use macroquad::window;
use menu::GameMode;
use tracing::error;

#[derive(PartialEq)]
enum LevelAction {
    UserWantsToStop,
    AllLevelsComplete,
    LoadNextLevel,
}

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
    let font = match load_ttf_font_from_bytes(include_bytes!("../assets/FiraSans-Black.ttf")) {
        Ok(font) => font,
        Err(err) => {
            error!(?err, "Failed to load text font");
            render_error_message(&format!("Failed to load text font: {err}")).await;
            panic!()
        }
    };
    let cx = Context { font };

    loop {
        next_frame().await;
        let game_mode = menu::start(&cx).await;

        match game_mode {
            GameMode::EndlessGame => {
                start_game(&cx, &Level::default()).await;
            }
            GameMode::Levels => {
                if play_custom_levels(&cx).await == LevelAction::UserWantsToStop {
                    continue;
                }
                if play_base_levels(&cx).await == LevelAction::UserWantsToStop {
                    continue;
                }
            }
            GameMode::Exit => break,
        }
    }
}

async fn play_custom_levels(cx: &Context) -> LevelAction {
    let custom_level_names = match search_for_custom_levels(LEVEL_PATH) {
        Ok(paths) => paths,
        Err(err) => {
            error!(?err, "Failed to search for custom levels");
            vec![]
        }
    };
    for level_name in &custom_level_names {
        match Level::load_level(LEVEL_PATH, level_name) {
            Ok(level) => {
                if loop_level(cx, &level).await == LevelAction::UserWantsToStop {
                    return LevelAction::UserWantsToStop;
                }
            }
            Err(err) => {
                error!(
                    ?err,
                    "Custom level {} is not valid and therefore skipped", level_name
                );
            }
        };
    }
    LevelAction::AllLevelsComplete
}

async fn play_base_levels(cx: &Context) -> LevelAction {
    for level in base_levels() {
        if loop_level(cx, &level).await == LevelAction::UserWantsToStop {
            return LevelAction::UserWantsToStop;
        }
    }
    LevelAction::AllLevelsComplete
}

async fn loop_level(cx: &Context, level: &Level) -> LevelAction {
    loop {
        match start_game(cx, level).await {
            GameOutcome::Win => return LevelAction::LoadNextLevel,
            GameOutcome::Exit => return LevelAction::UserWantsToStop,
            GameOutcome::Lose => {}
        }
    }
}
