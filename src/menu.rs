use crate::constants::{
    BACKGROUND_COLOR, OBSTACLE_COLOR, OPTION_TEXT_SIZE, SNAKE_HEAD_COLOR, TITLE_TEXT_SIZE,
};
use crate::graphic_utils::{draw_x_centered_rect, render_text};
use crate::Context;
use euclid::Point2D;
use macroquad::input::{get_last_key_pressed, KeyCode};
use macroquad::prelude::{clear_background, next_frame};
use macroquad::window::{screen_height, screen_width};

pub struct Menu {
    cursor: i32,
}

const NUMBER_OF_OPTIONS: i32 = 3;
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameMode {
    EndlessGame,
    Levels,
    Exit,
}

impl Menu {
    fn render_menu(&mut self, cx: &Context) {
        clear_background(BACKGROUND_COLOR);
        let height_segment = screen_height() / (NUMBER_OF_OPTIONS + 1) as f32;
        self.render_boxes(height_segment);
        Menu::render_text(height_segment, cx);
    }

    fn render_boxes(&mut self, height_segment: f32) {
        for i in 0..NUMBER_OF_OPTIONS {
            draw_x_centered_rect(
                height_segment * (i as f32 + NUMBER_OF_OPTIONS as f32 / 2.0),
                height_segment / 2.0,
                OBSTACLE_COLOR,
            );
        }

        if self.cursor >= 0 && self.cursor < NUMBER_OF_OPTIONS {
            draw_x_centered_rect(
                height_segment * (self.cursor as f32 + NUMBER_OF_OPTIONS as f32 / 2.0),
                height_segment / 2.0,
                SNAKE_HEAD_COLOR,
            );
        }
    }

    fn render_text(height_segment: f32, cx: &Context) {
        let text: [&str; 4] = ["Rusty Head Snake", "Levels", "Endless Game", "Exit"];

        for i in 0..=NUMBER_OF_OPTIONS {
            let (color, font_size) = match i {
                0 => (SNAKE_HEAD_COLOR, TITLE_TEXT_SIZE),
                _ => (BACKGROUND_COLOR, OPTION_TEXT_SIZE),
            };

            render_text(
                #[allow(clippy::cast_sign_loss)]
                text[i as usize],
                Point2D::new(
                    screen_width() / 2.0,
                    height_segment * i as f32 + height_segment / 2.0,
                ),
                Some(&cx.font),
                font_size,
                color,
            );
        }
    }

    fn handle_key_press(&mut self, key: Option<KeyCode>) -> Option<GameMode> {
        if let Some(key) = key {
            match key {
                KeyCode::Up | KeyCode::W => {
                    self.cursor = (self.cursor - 1) % NUMBER_OF_OPTIONS;
                    if self.cursor < 0 {
                        self.cursor = NUMBER_OF_OPTIONS - 1;
                    }
                }
                KeyCode::Down | KeyCode::S => {
                    self.cursor = (self.cursor + 1) % NUMBER_OF_OPTIONS;
                }
                KeyCode::Space | KeyCode::Enter => {
                    return match self.cursor {
                        0 => Some(GameMode::Levels),
                        1 => Some(GameMode::EndlessGame),
                        2 => Some(GameMode::Exit),
                        _ => None,
                    };
                }
                _ => {}
            }
        }
        None
    }
}

pub async fn start(cx: &Context) -> GameMode {
    let mut menu = Menu { cursor: 0 };
    menu_loop(&mut menu, cx).await
}

pub async fn menu_loop(menu: &mut Menu, cx: &Context) -> GameMode {
    loop {
        menu.render_menu(cx);
        if let Some(game_mode) = menu.handle_key_press(get_last_key_pressed()) {
            return game_mode;
        }
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_CURSOR_POSITION: i32 = 0;

    fn init() -> Menu {
        Menu {
            cursor: INITIAL_CURSOR_POSITION,
        }
    }

    fn key_event(menu: &mut Menu, keys: KeyCode, expected_result: Option<GameMode>) {
        assert_eq!(expected_result, menu.handle_key_press(Some(keys)));
    }

    fn press_space_or_enter(menu: &mut Menu, game_mode: GameMode) {
        key_event(menu, KeyCode::Space, Some(game_mode));
        key_event(menu, KeyCode::Enter, Some(game_mode));
    }

    #[test_case::test_case(KeyCode::Up, true)]
    #[test_case::test_case(KeyCode::W, true)]
    #[test_case::test_case(KeyCode::Down, false)]
    #[test_case::test_case(KeyCode::S, false)]
    fn key_press_direction(key: KeyCode, reverse_expected_cursor_position: bool) {
        let mut menu = init();

        let mut expected_cursor_position: Vec<i32> = (0..NUMBER_OF_OPTIONS).collect();
        expected_cursor_position.push(INITIAL_CURSOR_POSITION);

        let mut cursor_position = vec![menu.cursor];
        for _ in 0..NUMBER_OF_OPTIONS {
            key_event(&mut menu, key, None);
            cursor_position.push(menu.cursor);
        }

        if reverse_expected_cursor_position {
            expected_cursor_position.reverse();
        }
        assert_eq!(expected_cursor_position, cursor_position);
    }

    #[test]
    fn key_press_result() {
        let mut menu = init();

        let expected_cursor_position = vec![0, 1, 0, NUMBER_OF_OPTIONS - 1, 0, 0];
        let mut cursor_position = vec![];

        press_space_or_enter(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, KeyCode::Down, None);
        press_space_or_enter(&mut menu, GameMode::EndlessGame);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, KeyCode::Up, None);
        press_space_or_enter(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, KeyCode::W, None);
        press_space_or_enter(&mut menu, GameMode::Exit);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, KeyCode::S, None);
        press_space_or_enter(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, KeyCode::X, None);
        press_space_or_enter(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);

        assert_eq!(expected_cursor_position, cursor_position);
    }
}
