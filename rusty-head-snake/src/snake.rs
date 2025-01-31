use crate::constants;

use constants::{SCALING_FACTOR, SNAKE_HEAD_COLOR, SNAKE_TAIL_COLOR, SNAKE_WIDTH};
use euclid::Point2D;
use graphics::{rectangle, Transformed};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub position: Vec<Point2D<f64, f64>>,
    pub direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            position: vec![Point2D::new(0.0, 0.0)],
            direction: Direction::Right,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, SNAKE_WIDTH * SCALING_FACTOR);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(self.position[0].x, self.position[0].y);
            rectangle(SNAKE_HEAD_COLOR, square, transform, gl);
        });

        for (i, position) in self.position[1..].iter().enumerate() {
            gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(position.x, position.y);
                let mut color = SNAKE_TAIL_COLOR;
                color[3] = 1.0 - (i + 1) as f32 / 20.0;
                rectangle(color, square, transform, gl);
            });
        }
    }
}
