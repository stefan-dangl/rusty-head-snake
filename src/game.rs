use crate::constants::{BACKGROUND_COLOR, OBSTACLE_COLOR, OBSTACLE_WIDTH};
use crate::graphic_utils::{rectangle_corners, transform};
use crate::level::Level;
use crate::snake::{Direction, Snake};
use crate::target::Target;
use crate::text::TextHandler;
use euclid::Point2D;
use glutin_window::GlutinWindow as Window;
use graphics::{clear, rectangle};
use opengl_graphics::GlGraphics;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::{ButtonArgs, ButtonEvent, EventLoop};

pub struct Game<'a> {
    pub snake: Snake,
    pub target: Target,
    pub obstacles: Vec<Point2D<i32, i32>>,
    pub width: i32,
    pub height: i32,
    pub text: TextHandler<'a>,
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
enum ButtonResult {
    Exit,
    None,
}

impl Game<'_> {
    fn render_game(
        &mut self,
        args: &RenderArgs,
        gl: &mut GlGraphics,
        point_counter: i32,
        point_target: Option<i32>,
    ) {
        let scaling = (
            args.window_size[0] / f64::from(self.width),
            args.window_size[1] / f64::from(self.height),
        );

        gl.draw(args.viewport(), |_, gl| {
            clear(BACKGROUND_COLOR, gl);
        });
        self.render_obstacles(args, gl, scaling);
        self.target.render(args, gl, scaling);
        self.snake.render(args, gl, scaling);
        self.text
            .render_points(args, gl, point_counter, point_target);
    }

    fn render_obstacles(&mut self, args: &RenderArgs, gl: &mut GlGraphics, scaling: (f64, f64)) {
        let shape = rectangle_corners(OBSTACLE_WIDTH, scaling);

        for position in &self.obstacles {
            gl.draw(args.viewport(), |c, gl| {
                let transform = transform(c, *position, scaling);
                rectangle(OBSTACLE_COLOR, shape, transform, gl);
            });
        }
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

    fn button_press(&mut self, args: &ButtonArgs) -> ButtonResult {
        if args.state == piston::ButtonState::Press {
            if let piston::Button::Keyboard(key) = args.button {
                match key {
                    piston::Key::Escape => return ButtonResult::Exit,
                    piston::Key::Up | piston::Key::W => {
                        self.snake.set_direction(Direction::Up);
                    }
                    piston::Key::Down | piston::Key::S => {
                        self.snake.set_direction(Direction::Down);
                    }
                    piston::Key::Left | piston::Key::A => {
                        self.snake.set_direction(Direction::Left);
                    }
                    piston::Key::Right | piston::Key::D => {
                        self.snake.set_direction(Direction::Right);
                    }
                    _ => {}
                }
            }
        }
        ButtonResult::None
    }
}

pub fn start_game(window: &mut Window, gl: &mut GlGraphics, level: &Level) -> GameOutcome {
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
        text: TextHandler::new(),
    };

    game_loop(
        &mut game,
        window,
        gl,
        level.target_points,
        level.frames_per_second,
    )
}

fn game_loop(
    game: &mut Game,
    window: &mut Window,
    gl: &mut GlGraphics,
    target_points: Option<i32>,
    frames_per_second: i32,
) -> GameOutcome {
    #[allow(clippy::cast_sign_loss)]
    let mut events = Events::new(EventSettings::new()).ups(frames_per_second as u64);
    let mut point_counter = 0;
    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            game.render_game(&args, gl, point_counter, target_points);
        }

        if let Some(_args) = e.update_args() {
            let update_result = game.update();
            match update_result {
                UpdateResult::Collision => {
                    break;
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
        }

        if let Some(args) = e.button_args() {
            if game.button_press(&args) == ButtonResult::Exit {
                return GameOutcome::Exit;
            }
        }
    }
    GameOutcome::Lose
}

#[cfg(test)]
mod tests {
    use super::*;
    use piston::*;
    use std::vec;

    fn init(
        snake: Snake,
        target: Target,
        obstacles: Vec<Point2D<i32, i32>>,
        width: i32,
        height: i32,
    ) -> Game<'static> {
        Game {
            snake,
            target,
            obstacles,
            width,
            height,
            text: TextHandler::new(),
        }
    }

    fn default_init() -> Game<'static> {
        let width = 10;
        let height = 10;

        Game {
            snake: Snake::new(None, None, width, height),
            target: Target::new(&[], width, height),
            obstacles: vec![],
            width,
            height,
            text: TextHandler::new(),
        }
    }

    fn key_event(game: &mut Game, state: ButtonState, key: Key, expected_result: ButtonResult) {
        let args = ButtonArgs {
            state,
            button: Button::Keyboard(key),
            scancode: None,
        };
        assert_eq!(expected_result, game.button_press(&args));
    }

    #[test]
    fn test_button_press() {
        let mut game = default_init();

        let buttons_none_result = vec![
            Key::Up,
            Key::Down,
            Key::Left,
            Key::Right,
            Key::W,
            Key::S,
            Key::A,
            Key::D,
        ];
        for button in buttons_none_result {
            key_event(&mut game, ButtonState::Press, button, ButtonResult::None);
            key_event(&mut game, ButtonState::Release, button, ButtonResult::None);
        }

        key_event(
            &mut game,
            ButtonState::Press,
            Key::Escape,
            ButtonResult::Exit,
        );
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
