use macroquad::prelude::Color;

pub const SNAKE_WIDTH: f32 = 1.0;
pub const TARGET_WIDTH: f32 = 1.0;
pub const OBSTACLE_WIDTH: f32 = 1.0;

pub const WINDOW_WIDTH: i32 = 500;
pub const WINDOW_HEIGHT: i32 = 500;

pub const BACKGROUND_COLOR: Color = Color::new(0.1, 0.1, 0.2, 1.0);
pub const OBSTACLE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
pub const TARGET_COLOR: Color = Color::new(0.0, 1.0, 0.0, 1.0);
pub const SNAKE_HEAD_COLOR: Color = Color::new(1.0, 0.5, 0.0, 1.0);
pub const SNAKE_TAIL_COLOR: Color = Color::new(0.8, 1.0, 0.0, 1.0);

pub const TITLE_TEXT_SIZE: u16 = 40;
pub const OPTION_TEXT_SIZE: u16 = 25;
pub const POINTS_TEXT_SIZE: u16 = 25;

pub const LEVEL_PATH: &str = "levels";
