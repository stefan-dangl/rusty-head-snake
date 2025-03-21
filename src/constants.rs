use macroquad::{math::Vec2, prelude::Color};

use crate::touch_fields::TouchField;

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
pub const TOUCH_BOUNDARY_INACTIVE_COLOR: Color = Color::new(0.3, 0.3, 0.4, 0.1);
pub const TOUCH_BOUNDARY_ACTIVE_COLOR: Color = Color::new(1.0, 0.3, 0.4, 0.3);

pub const TITLE_TEXT_SIZE: u16 = 40;
pub const OPTION_TEXT_SIZE: u16 = 25;
pub const POINTS_TEXT_SIZE: u16 = 25;
pub const ERROR_TEXT_SIZE: u16 = 30;

pub const LEVEL_PATH: &str = "levels";

pub const TOUCH_BOUNDARY_INACTIVE_THICKNESS: f32 = 2.0;
pub const TOUCH_BOUNDARY_ACTIVE_THICKNESS: f32 = 10.0;
pub const UP_TOUCH_FIELD: TouchField = TouchField {
    p1: Vec2 { x: -1.0, y: -1.0 },
    p2: Vec2 { x: 1.0, y: 0.35 },
};
pub const DOWN_TOUCH_FIELD: TouchField = TouchField {
    p1: Vec2 { x: -0.5, y: 0.35 },
    p2: Vec2 { x: 0.5, y: 1.0 },
};
pub const LEFT_TOUCH_FIELD: TouchField = TouchField {
    p1: Vec2 { x: -1.0, y: 0.35 },
    p2: Vec2 { x: -0.5, y: 1.0 },
};
pub const RIGHT_TOUCH_FIELD: TouchField = TouchField {
    p1: Vec2 { x: 0.5, y: 0.35 },
    p2: Vec2 { x: 1.0, y: 1.0 },
};
