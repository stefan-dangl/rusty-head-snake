use crate::constants::{
    BACKGROUND_COLOR, OBSTACLE_COLOR, OPTION_TEXT_SIZE, SNAKE_HEAD_COLOR, TITLE_TEXT_SIZE,
};
use crate::graphic_utils::{render_text, render_x_centered_rect};
use crate::Context;
use euclid::Point2D;
use macroquad::input::{
    get_last_key_pressed, is_mouse_button_released, mouse_position, touches, KeyCode, MouseButton,
    Touch, TouchPhase,
};
use macroquad::math::Vec2;
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

#[derive(PartialEq, Debug)]
enum TouchMouseEvent {
    Floating,
    Enter,
    None,
}

#[derive(Debug)]
struct TouchMouseData {
    event: TouchMouseEvent,
    position: Vec2,
}

impl TouchMouseData {
    fn from_mouse_event(is_mouse_release: bool, position: (f32, f32)) -> Self {
        let event = if is_mouse_release {
            TouchMouseEvent::Enter
        } else {
            TouchMouseEvent::Floating
        };
        let position = Vec2::new(position.0, position.1);
        TouchMouseData { event, position }
    }

    fn from_touch_event(touch: &Touch) -> Self {
        let event = match touch.phase {
            TouchPhase::Started => TouchMouseEvent::Floating,
            TouchPhase::Ended => TouchMouseEvent::Enter,
            _ => TouchMouseEvent::None,
        };
        TouchMouseData {
            event,
            position: touch.position,
        }
    }
}

impl Menu {
    fn height_segment() -> f32 {
        screen_height() / (NUMBER_OF_OPTIONS + 1) as f32
    }

    fn render_menu(&mut self, cx: &Context, height_segment: f32) {
        clear_background(BACKGROUND_COLOR);
        self.render_boxes(height_segment);
        Menu::render_text(height_segment, cx);
    }

    fn render_boxes(&mut self, height_segment: f32) {
        for i in 0..NUMBER_OF_OPTIONS {
            render_x_centered_rect(
                height_segment * (i as f32 + NUMBER_OF_OPTIONS as f32 / 2.0),
                height_segment / 2.0,
                OBSTACLE_COLOR,
            );
        }

        if self.cursor >= 0 && self.cursor < NUMBER_OF_OPTIONS {
            render_x_centered_rect(
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

    fn game_mode_from_cursor_position(&self) -> Option<GameMode> {
        match self.cursor {
            0 => Some(GameMode::Levels),
            1 => Some(GameMode::EndlessGame),
            2 => Some(GameMode::Exit),
            _ => None,
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
                KeyCode::Space | KeyCode::Enter => return self.game_mode_from_cursor_position(),
                _ => {}
            }
        }
        None
    }

    fn handle_touch_mouse(
        &mut self,
        touch_mouse: &TouchMouseData,
        height_segment: f32,
    ) -> Option<GameMode> {
        if touch_mouse.event == TouchMouseEvent::Floating {
            for i in 0..NUMBER_OF_OPTIONS {
                let center_position = height_segment * (i as f32 + NUMBER_OF_OPTIONS as f32 / 2.0);
                if touch_mouse.position.y > center_position - height_segment / 4.0
                    && touch_mouse.position.y < center_position + height_segment / 4.0
                {
                    self.cursor = i;
                }
            }
        }

        if touch_mouse.event == TouchMouseEvent::Enter {
            let center_position =
                height_segment * (self.cursor as f32 + NUMBER_OF_OPTIONS as f32 / 2.0);
            if touch_mouse.position.y > center_position - height_segment / 4.0
                && touch_mouse.position.y < center_position + height_segment / 4.0
            {
                return self.game_mode_from_cursor_position();
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
        let height_segment = Menu::height_segment();

        if let Some(game_mode) = menu.handle_key_press(get_last_key_pressed()) {
            return game_mode;
        }

        for touch in touches() {
            let touch = TouchMouseData::from_touch_event(&touch);
            if let Some(game_mode) = menu.handle_touch_mouse(&touch, height_segment) {
                return game_mode;
            }
        }

        let mouse = TouchMouseData::from_mouse_event(
            is_mouse_button_released(MouseButton::Left),
            mouse_position(),
        );
        if let Some(game_mode) = menu.handle_touch_mouse(&mouse, height_segment) {
            return game_mode;
        }

        menu.render_menu(cx, height_segment);

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

    #[test]
    fn test_handle_touch() {
        let mut menu = init();
        let height_segment = 1.0;

        for i in 0..NUMBER_OF_OPTIONS {
            let position = i as f32 + NUMBER_OF_OPTIONS as f32 / 2.0;
            let mut touch = Touch {
                id: 0,
                phase: TouchPhase::Started,
                position: Vec2 {
                    x: 0.0,
                    y: position,
                },
            };

            let touch_mouse_data = TouchMouseData::from_touch_event(&touch.clone());
            assert_eq!(
                menu.handle_touch_mouse(&touch_mouse_data, height_segment),
                None
            );
            assert_eq!(menu.cursor, i);

            touch.phase = TouchPhase::Ended;
            let touch_mouse_data = TouchMouseData::from_touch_event(&touch);
            let selected_game_mode = menu.handle_touch_mouse(&touch_mouse_data, height_segment);
            assert_eq!(selected_game_mode, menu.game_mode_from_cursor_position());
        }
    }

    #[test]
    fn test_handle_mouse() {
        let mut menu = init();
        let height_segment = 1.0;

        for i in 0..NUMBER_OF_OPTIONS {
            let position = i as f32 + NUMBER_OF_OPTIONS as f32 / 2.0;

            let touch_mouse_data = TouchMouseData::from_mouse_event(false, (0.0, position));
            assert_eq!(
                menu.handle_touch_mouse(&touch_mouse_data, height_segment),
                None
            );
            assert_eq!(menu.cursor, i);

            let touch_mouse_data = TouchMouseData::from_mouse_event(true, (0.0, position));
            let selected_game_mode = menu.handle_touch_mouse(&touch_mouse_data, height_segment);
            assert_eq!(selected_game_mode, menu.game_mode_from_cursor_position());
        }
    }
}
