use crate::constants::{
    BACKGROUND_COLOR, DOWN_TOUCH_FIELD, LEFT_TOUCH_FIELD, OBSTACLE_COLOR, OBSTACLE_WIDTH,
    RIGHT_TOUCH_FIELD, UP_TOUCH_FIELD,
};
use crate::graphic_utils::{render_points, render_scaled_square};
use crate::level::Level;
use crate::snake::{Direction, Snake};
use crate::target::Target;
use crate::Context;
use euclid::Point2D;
use macroquad::input::{get_last_key_pressed, touches_local, KeyCode, Touch};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

pub struct Game {
    pub snake: Snake,
    pub target: Target,
    pub obstacles: Vec<Point2D<i32, i32>>,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq)]
pub enum GameOutcome {
    Lose,
    Win,
    Exit,
}

#[derive(PartialEq, Debug)]
enum UpdateResult {
    None,
    Collision,
    TargetHit,
}

#[derive(PartialEq, Debug)]
enum KeyPressResult {
    Exit,
    None,
}

impl Game {
    fn render_game(&mut self, cx: &Context, point_counter: i32, point_target: Option<i32>) {
        clear_background(BACKGROUND_COLOR);

        let scaling = (
            screen_width() / self.width as f32,
            screen_height() / self.height as f32,
        );

        self.render_obstacles(scaling);
        self.target.render(scaling);
        self.snake.render(scaling);
        render_points(point_counter, point_target, Some(&cx.font));
        Game::render_touch_field_boundaries();
    }

    fn render_obstacles(&mut self, scaling: (f32, f32)) {
        for position in &self.obstacles {
            render_scaled_square(OBSTACLE_COLOR, *position, OBSTACLE_WIDTH, scaling);
        }
    }

    fn render_touch_field_boundaries() {
        UP_TOUCH_FIELD.render_inactive_boundaries();
        DOWN_TOUCH_FIELD.render_inactive_boundaries();
        LEFT_TOUCH_FIELD.render_inactive_boundaries();
        RIGHT_TOUCH_FIELD.render_inactive_boundaries();
    }

    fn update(&mut self) -> UpdateResult {
        let snake_hit_target = self.snake_hit_target();
        self.snake
            .move_snake(snake_hit_target, self.width, self.height);
        if self.snake.is_overlapping() || self.snake_hit_obstacle() {
            UpdateResult::Collision
        } else if snake_hit_target {
            UpdateResult::TargetHit
        } else {
            UpdateResult::None
        }
    }

    fn snake_hit_target(&mut self) -> bool {
        if self.snake.position.contains(&self.target.position) {
            self.target = Target::new(&self.obstacles, self.width, self.height);
            return true;
        }
        false
    }

    fn snake_hit_obstacle(&mut self) -> bool {
        if self.obstacles.contains(&self.snake.position[0]) {
            return true;
        }
        false
    }

    fn handle_key_press(&mut self, key: Option<KeyCode>) -> KeyPressResult {
        if let Some(key) = key {
            match key {
                KeyCode::Escape => return KeyPressResult::Exit,

                KeyCode::Up | KeyCode::W => {
                    UP_TOUCH_FIELD.render_active_boundaries();
                    self.snake.set_direction(Direction::Up);
                }
                KeyCode::Down | KeyCode::S => {
                    DOWN_TOUCH_FIELD.render_active_boundaries();
                    self.snake.set_direction(Direction::Down);
                }
                KeyCode::Left | KeyCode::A => {
                    LEFT_TOUCH_FIELD.render_active_boundaries();
                    self.snake.set_direction(Direction::Left);
                }
                KeyCode::Right | KeyCode::D => {
                    RIGHT_TOUCH_FIELD.render_active_boundaries();
                    self.snake.set_direction(Direction::Right);
                }
                _ => {}
            }
        }
        KeyPressResult::None
    }

    fn handle_touch(&mut self, touch: &Touch) {
        if UP_TOUCH_FIELD.in_touch_field(touch.position) {
            UP_TOUCH_FIELD.render_active_boundaries();
            self.snake.set_direction(Direction::Up);
        }
        if DOWN_TOUCH_FIELD.in_touch_field(touch.position) {
            DOWN_TOUCH_FIELD.render_active_boundaries();
            self.snake.set_direction(Direction::Down);
        }
        if LEFT_TOUCH_FIELD.in_touch_field(touch.position) {
            LEFT_TOUCH_FIELD.render_active_boundaries();
            self.snake.set_direction(Direction::Left);
        }
        if RIGHT_TOUCH_FIELD.in_touch_field(touch.position) {
            RIGHT_TOUCH_FIELD.render_active_boundaries();
            self.snake.set_direction(Direction::Right);
        }
    }
}

pub async fn start_game(cx: &Context, level: &Level) -> GameOutcome {
    let mut game = Game {
        snake: Snake::new(
            level.start_position,
            level.start_direction,
            level.width,
            level.height,
        ),
        target: Target::new(&level.obstacles, level.width, level.height),
        obstacles: level.obstacles.clone(),
        width: level.width,
        height: level.height,
    };

    game_loop(&mut game, cx, level.target_points, level.updates_per_second).await
}

async fn game_loop(
    game: &mut Game,
    cx: &Context,
    target_points: Option<i32>,
    updates_per_second: i32,
) -> GameOutcome {
    let expected_frame_time = 1.0 / updates_per_second as f32;
    let mut frame_time_accumulated = 0.0;
    let mut point_counter = 0;

    loop {
        game.render_game(cx, point_counter, target_points);

        if game.handle_key_press(get_last_key_pressed()) == KeyPressResult::Exit {
            return GameOutcome::Exit;
        }

        for touch in touches_local() {
            game.handle_touch(&touch);
        }

        if frame_time_accumulated >= expected_frame_time {
            match game.update() {
                UpdateResult::Collision => {
                    return GameOutcome::Lose;
                }
                UpdateResult::TargetHit => {
                    point_counter += 1;
                    if let Some(target) = target_points {
                        if point_counter >= target {
                            return GameOutcome::Win;
                        }
                    }
                }
                UpdateResult::None => {}
            }
            frame_time_accumulated = 0.0;
        }

        frame_time_accumulated += get_frame_time();
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    fn init(
        snake: Snake,
        target: Target,
        obstacles: Vec<Point2D<i32, i32>>,
        width: i32,
        height: i32,
    ) -> Game {
        Game {
            snake,
            target,
            obstacles,
            width,
            height,
        }
    }

    #[test]
    fn test_snake_hit_target() {
        let width = 5;
        let height = 5;

        let target = Target {
            position: Point2D::new(2, 2),
        };
        let snake = Snake::new(
            Some(Point2D::new(1, 2)),
            Some(Direction::Right),
            width,
            height,
        );
        let mut game = init(snake, target, vec![], width, height);

        assert_eq!(UpdateResult::None, game.update());
        assert_eq!(UpdateResult::TargetHit, game.update());
        assert_eq!(UpdateResult::None, game.update());
    }

    #[test]
    fn test_snake_hit_obstacle() {
        let width = 5;
        let height = 5;

        let target = Target {
            position: Point2D::new(0, 0),
        };
        let snake = Snake::new(
            Some(Point2D::new(0, 2)),
            Some(Direction::Right),
            width,
            height,
        );
        let obstacle = vec![Point2D::new(2, 2)];
        let mut game = init(snake, target, obstacle, width, height);

        assert_eq!(UpdateResult::None, game.update());
        assert_eq!(UpdateResult::Collision, game.update());
        assert_eq!(UpdateResult::None, game.update());
    }
}
