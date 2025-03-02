use crate::{
    constants::{SNAKE_HEAD_COLOR, SNAKE_TAIL_COLOR, SNAKE_WIDTH},
    graphic_utils::{rectangle_corners, transform},
};
use euclid::{approxord::max, Point2D};
use graphics::rectangle;
use num_enum::TryFromPrimitive;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use rand::{rng, seq::IndexedRandom};
use std::ops::Range;

const NUMBER_OF_DIRECTIONS: u8 = 4;
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone, TryFromPrimitive)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Snake {
    pub position: Vec<Point2D<i32, i32>>,
    direction: Direction,
    current_direction: Direction,
}

impl Snake {
    pub fn new(
        start_position: Option<Point2D<i32, i32>>,
        start_direction: Option<Direction>,
        width: i32,
        height: i32,
    ) -> Self {
        Snake::new_inner(
            start_position,
            start_direction,
            0..width,
            0..height,
            0..NUMBER_OF_DIRECTIONS,
        )
    }

    fn new_inner(
        start_position: Option<Point2D<i32, i32>>,
        start_direction: Option<Direction>,
        width: Range<i32>,
        height: Range<i32>,
        direction_range: Range<u8>,
    ) -> Self {
        let position = start_position.unwrap_or_else(|| {
            let width_values: Vec<i32> = width.collect();
            let height_values: Vec<i32> = height.collect();
            Point2D::new(
                width_values.choose(&mut rng()).copied().unwrap(),
                height_values.choose(&mut rng()).copied().unwrap(),
            )
        });

        let direction = start_direction.unwrap_or_else(|| {
            let direction_values: Vec<u8> = direction_range.collect();
            let direction_value = direction_values.choose(&mut rng()).copied().unwrap();
            Direction::try_from(direction_value).unwrap()
        });

        Snake {
            position: vec![position],
            direction,
            current_direction: direction,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.current_direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            Direction::Down => {
                if self.current_direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
            Direction::Left => {
                if self.current_direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            Direction::Right => {
                if self.current_direction != Direction::Left {
                    self.direction = Direction::Right;
                }
            }
        }
    }

    pub fn move_snake(&mut self, snake_hit_target: bool, width: i32, height: i32) {
        self.current_direction = self.direction;
        self.propagate_position(snake_hit_target);
        self.adjust_head(width, height);
    }

    fn propagate_position(&mut self, keep_tail: bool) {
        let tail = self.position[self.position.len() - 1];
        for i in (1..self.position.len()).rev() {
            self.position[i] = self.position[i - 1];
        }
        if keep_tail {
            self.position.push(tail);
        }
    }

    fn adjust_head(&mut self, width: i32, height: i32) {
        match self.direction {
            Direction::Up => {
                self.position[0].y = (self.position[0].y + height - 1) % height;
            }
            Direction::Down => {
                self.position[0].y = (self.position[0].y + 1) % height;
            }
            Direction::Left => {
                self.position[0].x = (self.position[0].x + width - 1) % width;
            }
            Direction::Right => {
                self.position[0].x = (self.position[0].x + 1) % width;
            }
        }
    }

    pub fn is_overlapping(&self) -> bool {
        self.position[1..].contains(&self.position[0])
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics, scaling: (f64, f64)) {
        let shape = rectangle_corners(SNAKE_WIDTH, scaling);

        gl.draw(args.viewport(), |c, gl| {
            let transform = transform(c, self.position[0], scaling);
            rectangle(SNAKE_HEAD_COLOR, shape, transform, gl);
        });

        for (i, position) in self.position[1..].iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            gl.draw(args.viewport(), |c, gl| {
                let transform = transform(c, *position, scaling);
                let mut color = SNAKE_TAIL_COLOR;
                color[3] = max(1.0 - i as f32 * 0.075, 0.25);
                rectangle(color, shape, transform, gl);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_snake() {
        let start_position = Point2D::new(5, 5);
        let start_direction = Direction::Right;

        let expected_snake = Snake {
            position: vec![start_position],
            direction: start_direction,
            current_direction: start_direction,
        };
        let snake = Snake::new_inner(
            Some(start_position),
            Some(start_direction),
            Range { start: 0, end: 10 },
            Range { start: 0, end: 10 },
            Range {
                start: 0,
                end: NUMBER_OF_DIRECTIONS,
            },
        );
        assert_eq!(expected_snake, snake);
    }

    #[test]
    fn new_snake_random_start_position() {
        let start_position = Point2D::new(3, 5);
        let start_direction = Direction::Right;

        let expected_snake = Snake {
            position: vec![start_position],
            direction: start_direction,
            current_direction: start_direction,
        };
        let snake = Snake::new_inner(
            None,
            Some(start_direction),
            Range {
                start: start_position.x,
                end: start_position.x + 1,
            },
            Range {
                start: start_position.y,
                end: start_position.y + 1,
            },
            Range {
                start: 0,
                end: NUMBER_OF_DIRECTIONS,
            },
        );
        assert_eq!(expected_snake, snake);
    }

    #[test]
    fn new_snake_random_start_direction() {
        let start_position = Point2D::new(5, 5);
        let start_direction = Direction::Left;

        let expected_snake = Snake {
            position: vec![start_position],
            direction: start_direction,
            current_direction: start_direction,
        };
        let snake = Snake::new_inner(
            Some(start_position),
            None,
            Range { start: 0, end: 10 },
            Range { start: 0, end: 10 },
            Range {
                start: start_direction as u8,
                end: start_direction as u8 + 1,
            },
        );
        assert_eq!(expected_snake, snake);
    }

    fn move_snake(
        positions: &[Vec<Point2D<i32, i32>>],
        directions: &[Direction],
        target_hit: &[bool],
        width: i32,
        height: i32,
    ) {
        let mut snake = Snake {
            position: positions[0].to_vec(),
            direction: directions[0],
            current_direction: directions[0],
        };

        for i in 1..directions.len() {
            snake.move_snake(target_hit[i - 1], width, height);
            snake.set_direction(directions[i]);
            let expected_snake = Snake {
                position: positions[i].to_vec(),
                direction: directions[i],
                current_direction: directions[i - 1],
            };
            assert_eq!(snake, expected_snake);
        }
    }

    #[test]
    fn move_snake_right() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(0, 0)],
            vec![Point2D::new(1, 0), Point2D::new(0, 0)],
            vec![Point2D::new(2, 0), Point2D::new(1, 0)],
            vec![Point2D::new(0, 0), Point2D::new(2, 0), Point2D::new(1, 0)],
            vec![Point2D::new(1, 0), Point2D::new(0, 0), Point2D::new(2, 0)],
        ];
        let directions = [Direction::Right; 5];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn move_snake_left() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(2, 0)],
            vec![Point2D::new(1, 0), Point2D::new(2, 0)],
            vec![Point2D::new(0, 0), Point2D::new(1, 0)],
            vec![Point2D::new(2, 0), Point2D::new(0, 0), Point2D::new(1, 0)],
            vec![Point2D::new(1, 0), Point2D::new(2, 0), Point2D::new(0, 0)],
        ];
        let directions = [Direction::Left; 5];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn move_snake_up() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(0, 0)],
            vec![Point2D::new(0, 2), Point2D::new(0, 0)],
            vec![Point2D::new(0, 1), Point2D::new(0, 2)],
            vec![Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(0, 2)],
            vec![Point2D::new(0, 2), Point2D::new(0, 0), Point2D::new(0, 1)],
        ];
        let directions = [Direction::Up; 5];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn move_snake_down() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(0, 0)],
            vec![Point2D::new(0, 1), Point2D::new(0, 0)],
            vec![Point2D::new(0, 2), Point2D::new(0, 1)],
            vec![Point2D::new(0, 0), Point2D::new(0, 2), Point2D::new(0, 1)],
            vec![Point2D::new(0, 1), Point2D::new(0, 0), Point2D::new(0, 2)],
        ];
        let directions = [Direction::Down; 5];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn move_snake_clockwise() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(0, 0)],
            vec![Point2D::new(1, 0), Point2D::new(0, 0)],
            vec![Point2D::new(1, 1), Point2D::new(1, 0)],
            vec![Point2D::new(0, 1), Point2D::new(1, 1), Point2D::new(1, 0)],
            vec![Point2D::new(0, 0), Point2D::new(0, 1), Point2D::new(1, 1)],
        ];
        let directions = [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn move_snake_counter_clockwise() {
        let width = 3;
        let height = 3;
        let positions: [Vec<Point2D<i32, i32>>; 5] = [
            vec![Point2D::new(2, 0)],
            vec![Point2D::new(1, 0), Point2D::new(2, 0)],
            vec![Point2D::new(1, 1), Point2D::new(1, 0)],
            vec![Point2D::new(2, 1), Point2D::new(1, 1), Point2D::new(1, 0)],
            vec![Point2D::new(2, 0), Point2D::new(2, 1), Point2D::new(1, 1)],
        ];
        let directions = [
            Direction::Left,
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
        ];
        let target_hit: [bool; 4] = [true, false, true, false];
        move_snake(&positions, &directions, &target_hit, width, height)
    }

    #[test]
    fn overlapping() {
        let expected_snake = Snake {
            position: vec![
                Point2D::new(3, 0),
                Point2D::new(3, 1),
                Point2D::new(2, 1),
                Point2D::new(2, 0),
                Point2D::new(3, 0),
            ],
            direction: Direction::Up,
            current_direction: Direction::Up,
        };
        assert!(expected_snake.is_overlapping());
    }

    #[test]
    fn not_overlapping() {
        let expected_snake = Snake {
            position: vec![
                Point2D::new(3, 0),
                Point2D::new(3, 1),
                Point2D::new(2, 1),
                Point2D::new(2, 0),
            ],
            direction: Direction::Up,
            current_direction: Direction::Up,
        };
        assert!(!expected_snake.is_overlapping());
    }
}
