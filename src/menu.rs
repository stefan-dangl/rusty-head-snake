use crate::constants::{
    BACKGROUND_COLOR, OBSTACLE_COLOR, OPTION_TEXT_SIZE, SNAKE_HEAD_COLOR, TITLE_TEXT_SIZE,
};
use crate::text::TextHandler;
use euclid::Point2D;
use glutin_window::GlutinWindow as Window;
use graphics::{clear, rectangle};
use opengl_graphics::GlGraphics;
use piston::{
    event_loop::{EventSettings, Events},
    input::{RenderArgs, RenderEvent},
    Button, ButtonArgs, ButtonEvent, ButtonState, Key,
};

pub struct Menu<'a> {
    cursor: i32,
    text_handler: TextHandler<'a>,
}

const NUMBER_OF_OPTIONS: i32 = 3;
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameMode {
    EndlessGame,
    Levels,
    Exit,
}

impl Menu<'_> {
    fn render_game(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |_, gl| {
            clear(BACKGROUND_COLOR, gl);
        });

        let height_segment = args.window_size[1] / f64::from(NUMBER_OF_OPTIONS + 1);
        self.render_boxes(args, gl, height_segment);
        self.render_text(args, gl, height_segment);
    }

    fn render_boxes(&mut self, args: &RenderArgs, gl: &mut GlGraphics, height_segment: f64) {
        for i in 0..NUMBER_OF_OPTIONS {
            Menu::render_box(
                args,
                gl,
                height_segment * (f64::from(i) + f64::from(NUMBER_OF_OPTIONS) / 2.0),
                height_segment / 4.0,
                OBSTACLE_COLOR,
            );
        }

        if self.cursor >= 0 && self.cursor < NUMBER_OF_OPTIONS {
            Menu::render_box(
                args,
                gl,
                height_segment * (f64::from(self.cursor) + f64::from(NUMBER_OF_OPTIONS) / 2.0),
                height_segment / 4.0,
                SNAKE_HEAD_COLOR,
            );
        }
    }

    fn render_box(
        args: &RenderArgs,
        gl: &mut GlGraphics,
        y_position: f64,
        height: f64,
        color: [f32; 4],
    ) {
        let rectangle_shape = rectangle::centered([
            args.window_size[0] / 2.0,
            y_position,
            args.window_size[0] / 4.0,
            height,
        ]);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            rectangle(color, rectangle_shape, transform, gl);
        });
    }

    fn render_text(&mut self, args: &RenderArgs, gl: &mut GlGraphics, height_segment: f64) {
        let text: [&str; 4] = ["Rusty Head Snake", "Levels", "Endless Game", "Exit"];

        for i in 0..=NUMBER_OF_OPTIONS {
            let (color, scale) = match i {
                0 => (SNAKE_HEAD_COLOR, TITLE_TEXT_SIZE),
                _ => (BACKGROUND_COLOR, OPTION_TEXT_SIZE),
            };
            #[allow(clippy::cast_sign_loss)]
            self.text_handler.render(
                args,
                gl,
                text[i as usize],
                Point2D::new(
                    args.window_size[0] / 2.0,
                    height_segment * f64::from(i) + height_segment / 2.0,
                ),
                scale,
                color,
            );
        }
    }

    fn button_press(&mut self, args: &ButtonArgs) -> Option<GameMode> {
        if args.state == ButtonState::Press {
            if let Button::Keyboard(key) = args.button {
                match key {
                    Key::Up | Key::W => {
                        self.cursor = (self.cursor - 1) % NUMBER_OF_OPTIONS;
                        if self.cursor < 0 {
                            self.cursor = NUMBER_OF_OPTIONS - 1;
                        }
                    }
                    Key::Down | Key::S => {
                        self.cursor = (self.cursor + 1) % NUMBER_OF_OPTIONS;
                    }
                    Key::Space | Key::Return => {
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
        }
        None
    }
}

pub fn start(window: &mut Window, gl: &mut GlGraphics) -> GameMode {
    let mut menu = Menu {
        cursor: 0,
        text_handler: TextHandler::new(),
    };
    menu_loop(&mut menu, window, gl)
}

pub fn menu_loop(app: &mut Menu, window: &mut Window, gl: &mut GlGraphics) -> GameMode {
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            app.render_game(&args, gl);
        }

        if let Some(args) = e.button_args() {
            if let Some(game_mode) = app.button_press(&args) {
                return game_mode;
            }
        }
    }
    GameMode::EndlessGame
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_CURSOR_POSITION: i32 = 0;

    fn init() -> Menu<'static> {
        Menu {
            cursor: INITIAL_CURSOR_POSITION,
            text_handler: TextHandler::new(),
        }
    }

    fn key_event(menu: &mut Menu, state: ButtonState, key: Key, expected_result: Option<GameMode>) {
        let args = ButtonArgs {
            state,
            button: Button::Keyboard(key),
            scancode: None,
        };
        assert_eq!(expected_result, menu.button_press(&args));
    }

    #[test_case::test_case(Key::Up, true)]
    #[test_case::test_case(Key::W, true)]
    #[test_case::test_case(Key::Down, false)]
    #[test_case::test_case(Key::S, false)]
    fn button_press_direction(key: Key, reverse_expected_cursor_position: bool) {
        let mut menu = init();

        let mut expected_cursor_position: Vec<i32> = (0..NUMBER_OF_OPTIONS).collect();
        expected_cursor_position.push(INITIAL_CURSOR_POSITION);

        let mut cursor_position = vec![menu.cursor];
        for _ in 0..NUMBER_OF_OPTIONS {
            key_event(&mut menu, ButtonState::Press, key, None);
            cursor_position.push(menu.cursor);
        }

        if reverse_expected_cursor_position {
            expected_cursor_position.reverse();
        }
        assert_eq!(expected_cursor_position, cursor_position);
    }

    fn press_space_and_return(menu: &mut Menu, game_mode: GameMode) {
        key_event(menu, ButtonState::Press, Key::Space, Some(game_mode));
        key_event(menu, ButtonState::Press, Key::Return, Some(game_mode));
    }

    #[test]
    fn button_press_alternating() {
        let mut menu = init();

        let expected_cursor_position = vec![0, 1, 0, NUMBER_OF_OPTIONS - 1, 0];
        let mut cursor_position = vec![];

        press_space_and_return(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, ButtonState::Press, Key::Down, None);
        press_space_and_return(&mut menu, GameMode::EndlessGame);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, ButtonState::Press, Key::Up, None);
        press_space_and_return(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, ButtonState::Press, Key::W, None);
        press_space_and_return(&mut menu, GameMode::Exit);
        cursor_position.push(menu.cursor);
        key_event(&mut menu, ButtonState::Press, Key::S, None);
        press_space_and_return(&mut menu, GameMode::Levels);
        cursor_position.push(menu.cursor);

        assert_eq!(expected_cursor_position, cursor_position);
    }

    #[test]
    fn button_release() {
        let mut menu = init();

        key_event(&mut menu, ButtonState::Release, Key::Up, None);
        assert_eq!(INITIAL_CURSOR_POSITION, menu.cursor);
        key_event(&mut menu, ButtonState::Release, Key::Down, None);
        assert_eq!(INITIAL_CURSOR_POSITION, menu.cursor);
        key_event(&mut menu, ButtonState::Release, Key::Space, None);
        assert_eq!(INITIAL_CURSOR_POSITION, menu.cursor);
    }
}
